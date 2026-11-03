use std::{fs, path::PathBuf, str::FromStr};

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use clap::{Command, arg, command, value_parser};
use notify::{RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use insite::{compiler::Compiler, config::BuildConfig, error::Error, lexer::Lexer, parser::Parser};

#[tokio::main]
async fn main() {
    let filename = PathBuf::from_str("index.site").unwrap();

    // check for existing files first
    if !filename.exists() {
        println!("insite: unable to locate 'index.site' build file");
        return;
    }

    let config_file = PathBuf::from_str("insite.toml").unwrap();

    if !config_file.exists() {
        println!("insite: unable to locate 'insite.toml' configuration file");
        return;
    }

    let config = match BuildConfig::from_toml(&fs::read_to_string(&config_file).unwrap()) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let matches = command!()
        .subcommand(Command::new("build").about("compile and build a site"))
        .subcommand(Command::new("dev").about("start a live reloading web server"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("build")
        && let Err(e) = build(&filename, &config, false)
    {
        println!("{e}");
    } else if let Some(_) = matches.subcommand_matches("dev")
        && let Err(e) = dev(&filename, &config_file, &config).await
    {
        println!("{e}");
    }
}

/// Compile an `index.site` file by initializing a `Lexer`, `Parser`, and `Compiler`.
///
/// - `filename`: the filename of the file to compile
/// - `config_file`: the filename of the TOML configuration file
/// - `dev_mode`: if development mode should be enabled on the compiler
///
/// This function will additionally write encountered errors to standard output.
fn build(filename: &PathBuf, config: &BuildConfig, dev_mode: bool) -> Result<(), Error> {
    let contents = fs::read_to_string(&filename)?;

    let mut lexer = Lexer::new(&filename, &contents);
    let tokens = lexer.run()?;

    let mut parser = Parser::new(&filename, &config, &tokens);
    let commands = parser.parse()?;

    let mut compiler = Compiler::new(&config, dev_mode);
    compiler.build(commands)?;

    Ok(())
}

/// Start a live reloading web server and watch for changes on 'index.site' and 'insite.toml' data
/// (triggering re-compilations)
///
/// - `filename`: the filename of the file to compile
/// - `config_file`: the filename of the TOML configuration file
/// - `config`: the build configuration
async fn dev(filename: &PathBuf, config_file: &PathBuf, config: &BuildConfig) -> Result<(), Error> {
    let (reload_tx, _) = broadcast::channel::<()>(16);

    let server_tx = reload_tx.clone();

    let c = config.clone();
    tokio::spawn(async move {
        if let Err(e) = start_server(&c.output, server_tx).await {
            println!("insite: server error\n{e}");
        }
    });

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .map_err(|e| Error::General(e.to_string()))?;

    watcher
        .watch(filename, RecursiveMode::NonRecursive)
        .map_err(|e| Error::General(e.to_string()))?;
    watcher
        .watch(config_file, RecursiveMode::NonRecursive)
        .map_err(|e| Error::General(e.to_string()))?;

    if let Some(s) = &config.stylesheet {
        watcher
            .watch(&s, RecursiveMode::NonRecursive)
            .map_err(|e| Error::General(e.to_string()))?;
    }

    println!("insite: watching '{}'", filename.display());

    // initial compilation
    if let Err(e) = build(filename, config, true) {
        println!("{e}");
    };

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) if event.kind.is_modify() || event.kind.is_create() => {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                // re-parse the build config
                let config = BuildConfig::from_toml(&fs::read_to_string(config_file)?)?;

                println!("insite: compiling site");
                if let Err(e) = build(filename, &config, true) {
                    println!("{e}");
                };

                println!("insite: reloading site");
                let _ = reload_tx.send(());
            }
            Ok(_) => {}
            Err(e) => println!("insite: watch error '{e}'"),
        }
    }

    Ok(())
}

async fn websocket(
    ws: WebSocketUpgrade,
    State(tx): State<broadcast::Sender<()>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let mut rx = tx.subscribe();

        while rx.recv().await.is_ok() {
            if socket.send(Message::Text("reload".into())).await.is_err() {
                break;
            }
        }
    })
}

/// Start a webserver at `directory`.
async fn start_server(
    directory: &PathBuf,
    tx: broadcast::Sender<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/__insite_ws", get(websocket))
        .fallback_service(ServeDir::new(directory).append_index_html_on_directories(true))
        .with_state(tx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("insite: serving site at http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
