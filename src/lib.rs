mod cli;
mod naive;
mod smart;

use cli::parse_args;
use naive::generate_polycubes_naive;
use smart::generate_polycubes_smart;

pub fn generate_polycubes() {
    let args = parse_args();

    match args.algorithm.as_ref() {
        "naive" => generate_polycubes_naive(),
        "smart" => generate_polycubes_smart(),
        alg => println!("unknown algorithm {}", alg),
    }
}
