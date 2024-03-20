mod error;
mod eval;
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

    /// AST debug mode
    #[arg(long)]
    debug_ast: bool,
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn repl(options: Args) -> Result<(), Error> {
    print_prompt();

    // Line will be None if someone hits ^D
    for (lineno, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();

        if line.is_empty() {
            print_prompt();
            continue;
        }

        let tokens = scanner::scan(line, lineno as u32)?;

        match parser::parse(tokens) {
            Ok(ast) => {
                if options.debug_ast {
                    ast.pretty();
                }

                let val = ast.eval();
                match val {
                    Ok(v) => println!("{v:?}"),
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("{e}"),
        }

        print_prompt();
    }

    // Avoid leaving prompt printed w/o newline
    println!();
    Ok(())
}

fn read_file(options: Args) -> Result<(), Error> {
    let contents = fs::read_to_string(options.file.unwrap())
        .expect("Should have been able to read the file");

    let tokens = scanner::scan(&contents, 0)?;

    let ast = parser::parse(tokens)
        .map_err(|e| Error::new_with_msg(e, 0))?;
    let val = ast.eval();
    println!("{val:?}");

    Ok(())
}

fn main() {
    let args = Args::parse();
    let err = match args.file {
        Some(_) => read_file(args),
        None => repl(args),
    };

    if let Err(e) = err {
        println!("{}", e);
        std::process::exit(65);
    }
}
