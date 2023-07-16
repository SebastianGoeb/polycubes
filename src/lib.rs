mod cli;
mod poly_2d;
mod snake_2d;

use cli::parse_cli;
use poly_2d::generate_polys_2d;
use snake_2d::generate_snake_2d;

pub fn generate_polycubes() {
    let cli = parse_cli();

    match cli.command {
        cli::Commands::Snake2d { n } => generate_snake_2d(n),
        cli::Commands::Poly2d { max_n } => generate_polys_2d(max_n),
    }
}
