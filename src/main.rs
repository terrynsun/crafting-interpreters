mod error;
mod expr;
mod parser;
mod pretty;
mod scanner;
mod token;

use clap::Parser;

use std::fs;
use std::io::{self, Write};

use error::Error;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to run
    file: Option<String>,
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn repl() -> Result<(), Error> {
    print_prompt();

    // Line will be None if someone hits ^D
    for (lineno, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();

        let tokens = scanner::scan(line, lineno as u32);
        println!("{:?}", tokens);

        print_prompt();
    }

    // Avoid leaving prompt printed w/o newline
    println!();
    Ok(())
}

fn read_file(fname: String) -> Result<(), Error> {
    let contents = fs::read_to_string(fname).expect("Should have been able to read the file");

    let tokens = scanner::scan(contents, 0)?;
    println!("{:?}", tokens);

    let ast = parser::parse(tokens);
    ast.pretty();

    Ok(())
}

fn main() {
    let args = Args::parse();
    let err = match args.file {
        Some(fname) => read_file(fname),
        None => repl(),
    };

    if let Err(e) = err {
        println!("{}", e);
        std::process::exit(65);
    }
}
