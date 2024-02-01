use std::fmt::Display;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
    pub algorithm: Option<Algorithm>,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}


#[derive(Debug, ValueEnum, Clone)]
pub enum Algorithm {
    A32,
    B8,
}

impl FromStr for Algorithm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a32" => Ok(Algorithm::A32),
            "b8" => Ok(Algorithm::B8),
            _ => Err(())
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Algorithm::A32 => "A32",
            Algorithm::B8 => "B8"
        })
    }
}