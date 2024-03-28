mod config;
mod error;
mod eval;
mod exec;
mod expr;
mod parser;
mod pretty;
mod scanner;
mod token;

use std::fs;
use std::io::{self, Write};

use config::Config;
use error::ErrorState;
use exec::ExecState;

use clap::Parser;

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn repl(options: config::Config) -> Result<(), ErrorState> {
    print_prompt();

    let mut state = ExecState::new(options);

    // Line will be None if someone hits ^D
    for (lineno, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let mut line = line.trim().to_string();

        // Skip the line if there's only whitespace
        if line.is_empty() {
            print_prompt();
            continue;
        }

        // Helpfully append a semicolon to allow bare expressions in the repl.
        line.push(';');

        let tokens = match scanner::scan(&line, lineno as u32) {
            Ok(v) => v,
            Err(err) => {
                println!("{err}");
                print_prompt();
                continue;
            }
        };

        let program = match parser::parse(tokens) {
            Ok(program) => program,
            Err(err) => {
                println!("{err}");
                print_prompt();
                continue;
            }
        };

        let _ = state.exec(program).map_err(|e| println!("{e}"));

        print_prompt();
    }

    // Avoid leaving prompt printed w/o newline
    println!();
    Ok(())
}

fn process_file(options: Config) -> Result<(), ErrorState> {
    let contents = fs::read_to_string(options.file.clone().unwrap())
        .expect("Should have been able to read the file");

    let tokens = scanner::scan(&contents, 0)?;

    let program = parser::parse(tokens)?;

    let mut state = ExecState::new(options);

    let _ = state.exec(program).map_err(|e| println!("{e}"));

    Ok(())
}

fn main() {
    let args = Config::parse();
    let err = match args.file {
        Some(_) => process_file(args),
        None => repl(args),
    };

    if let Err(e) = err {
        println!("{}", e);
        std::process::exit(65);
    }
}
