use std::{io::{self, Write}, fs};

use clap::Parser;

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

fn repl()  {
    print_prompt();

    // line will be None if someone hits ^D
    for line in io::stdin().lines() {
        let line = line.unwrap();
        println!("{}", line.trim());

        print_prompt();
    }
}

fn read_file(fname: String) {
    let contents = fs::read_to_string(fname)
        .expect("Should have been able to read the file");

    for line in contents.split("\n") {
        let line = line.trim();

        println!("{}", line);
    }
}

fn main() {
    let args = Args::parse();
    match args.file {
        Some(fname) => {
            read_file(fname);
        },
        None => {
            repl()
        }
    }
}
