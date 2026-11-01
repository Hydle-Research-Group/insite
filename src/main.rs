use std::{fs, path::PathBuf, str::FromStr};

use clap::{Command, command};

use insite::{
    compiler::{BuildConfig, Compiler},
    lexer::Lexer,
    parser::Parser,
};

fn main() {
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

    let matches = command!()
        .subcommand(Command::new("build").about("compile and build a site"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("build") {
        build(&filename, &config_file);
    }
}

/// Compile an `index.site` file by initializing a `Lexer`, `Parser`, and `Compiler`.
///
/// - `filename`: the filename of the file to compile
/// - `config_file`: the filename of the TOML configuration file
///
/// This function will additionally write encountered errors to standard output.
fn build(filename: &PathBuf, config_file: &PathBuf) {
    let contents = fs::read_to_string(&filename).unwrap();

    let mut lexer = Lexer::new(&filename, &contents);
    let tokens = match lexer.run() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let mut parser = Parser::new(&filename, &tokens);
    let commands = match parser.parse() {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let config = match BuildConfig::from_toml(&fs::read_to_string(&config_file).unwrap()) {
        Ok(b) => b,
        Err(e) => {
            println!("insite: error parsing '{}'\n{e}", config_file.display());
            return;
        }
    };

    let mut compiler = Compiler::new();
    match compiler.build(config, commands) {
        Ok(()) => {}
        Err(e) => {
            println!("{e}");
            return;
        }
    };
}
