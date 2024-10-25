mod config;
mod error;
mod eval;
mod exec;
mod grammar;
mod parser;
mod pretty;
mod scanner;
mod token;

mod tests;

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

fn handle_line(line: &str, lineno: u32, state: &mut ExecState) -> Result<(), ErrorState> {
    let tokens = scanner::scan(&line, lineno)?;

    let program = parser::parse(tokens)?;

    state.exec(program)?;

    if !state.value.is_nil() {
        println!("{}", state.value);
    }

    Ok(())
}

fn repl(options: config::Config) -> Result<(), ErrorState> {
    print_prompt();

    let mut state = ExecState::new(options);

    // Line will be None if someone hits ^D
    for (lineno, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let mut line = line.trim().to_string();

        // Helpfully append a semicolon to allow bare expressions in the repl.
        if !line.ends_with(';') {
            line.push(';');
        }

        // Print error, rerun loop, never crash on repl error.
        if let Err(e) = handle_line(&line, lineno as u32, &mut state) {
            println!("{e}");
        };

        print_prompt();
    }

    // Print bare newline to gracefully avoid leaving the prompt printed in terminal w/o newline.
    println!();

    Ok(())
}

fn process_file(options: Config) -> Result<(), ErrorState> {
    let contents = fs::read_to_string(options.file.clone().unwrap())
        .expect("Should have been able to read the file");

    let tokens = scanner::scan(&contents, 0)?;

    let program = parser::parse(tokens)?;

    let mut state = ExecState::new(options);

    state.exec(program)
}

fn main() {
    let args = Config::parse();
    let err = match args.file {
        Some(_) => process_file(args),
        None => repl(args),
    };

    if let Err(e) = err {
        println!("{e}");
        std::process::exit(65);
    }
}
