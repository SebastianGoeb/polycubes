mod cli;
mod poly_2d;

use cli::parse_cli;
use poly_2d::poly::generate_polys;
use poly_2d::snake::generate_snake_2d;

pub fn generate_polycubes() {
    let cli = parse_cli();

    match cli.command {
        cli::Commands::Snake2d { n } => generate_snake_2d(n),
        cli::Commands::Poly2d(poly2d) => generate_polys(poly2d),
    }
}
