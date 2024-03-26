mod error;
mod eval;
mod expr;
mod parser;
mod pretty;
mod scanner;
mod token;

use clap::Parser;
use expr::{Program, Decl, Stmt};

use std::fs;
use std::io::{self, Write};

use error::ErrorState;

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

fn exec_program(program: Program, options: &Args) {
    for decl in program {
        match decl {
            Decl::VarDecl(_, _) => todo!(),
            Decl::Stmt(stmt) => {
                match stmt {
                    Stmt::Expr(e) => {
                        if options.debug_ast {
                            e.pretty();
                        }

                        let val = e.eval();
                        match val {
                            Ok(_v) => (),
                            Err(e) => println!("{e}"),
                        }
                    }
                    Stmt::Print(e) => {
                        if options.debug_ast {
                            e.pretty();
                        }

                        let val = e.eval();
                        match val {
                            Ok(v) => println!("{v:?}"),
                            Err(e) => println!("{e}"),
                        }
                    }
                }
            }
        }
    }
}

fn repl(options: Args) -> Result<(), ErrorState> {
    print_prompt();

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

        let tokens = scanner::scan(&line, lineno as u32)?;

        match parser::parse(tokens) {
            Ok(program) => {
                exec_program(program, &options);
            }
            Err(e) => println!("{e}"),
        }

        print_prompt();
    }

    // Avoid leaving prompt printed w/o newline
    println!();
    Ok(())
}

fn process_file(options: Args) -> Result<(), ErrorState> {
    let contents = fs::read_to_string(options.file.clone().unwrap())
        .expect("Should have been able to read the file");

    let tokens = scanner::scan(&contents, 0)?;

    let program = parser::parse(tokens)?;

    exec_program(program, &options);

    Ok(())
}

fn main() {
    let args = Args::parse();
    let err = match args.file {
        Some(_) => process_file(args),
        None => repl(args),
    };

    if let Err(e) = err {
        println!("{}", e);
        std::process::exit(65);
    }
}
