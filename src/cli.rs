use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Snake2d { n: usize },
    Poly2d { max_n: usize },
}

pub fn parse_cli() -> Cli {
    return Cli::parse();
}
