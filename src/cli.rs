use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generates random walks (snakes) in 2 dimensions
    Snake2d { n: usize },
    /// Generates polycubes in 2 dimensions
    Poly2d(Poly2d),
}

#[derive(Args, Debug)]
pub struct Poly2d {
    /// Generate Polycubes up to size
    pub max_n: usize,

    #[arg(short, long)]
    pub report_polys: bool,

    #[arg(short, long)]
    pub algorithm: Option<String>,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
