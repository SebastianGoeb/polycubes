use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub algorithm: String,

    #[arg(short, long)]
    pub max: Option<u32>,
}

pub fn parse_args() -> Args {
    return Args::parse();
}
