use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// File to run
    pub file: Option<String>,

    /// AST debug mode
    #[arg(long)]
    pub debug_ast: bool,
}
