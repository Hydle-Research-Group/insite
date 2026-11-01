use std::{fs, path::PathBuf, str::FromStr};

use clap::{ArgAction, Command, arg, command, value_parser};

use insite::{
    compiler::{BuildConfig, Compiler},
    lexer::Lexer,
    parser::Parser,
};

fn main() {
    let matches = command!()
        .subcommand(Command::new("build").about("compile and build a site"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("build") {
        let filename = PathBuf::from_str("index.site").unwrap();

        if !filename.exists() {
            println!("insite: unable to locate 'index.site' build file");
            return;
        }

        let config_file = PathBuf::from_str("insite.toml").unwrap();

        if !config_file.exists() {
            println!("insite: unable to locate 'insite.toml' configuration file");
            return;
        }

        let contents = fs::read_to_string(&filename).unwrap();

        let mut lexer = Lexer::new(&filename, &contents);
        let tokens = match lexer.run() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let mut parser = Parser::new(&filename, &tokens);
        let commands = match parser.parse() {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let mut compiler = Compiler::new();
        match compiler.build(
            BuildConfig::from_toml(&fs::read_to_string(&config_file).unwrap()).unwrap(),
            commands,
        ) {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }
}
