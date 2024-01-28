use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use lazy_static::lazy_static;
use nalgebra::Vector2;
use rayon::prelude::*;

use crate::cli::Poly2d;
use crate::poly_2d::shape::shape_with_grid::ShapeWithGrid;

static MOVES: &[Vector2<i32>] = &[
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
];

lazy_static! {
    static ref NUM: format_num::NumberFormat = format_num::NumberFormat::new();
}

pub fn generate_polys(cli: Poly2d) {
    println!("generating polycubes (in 2d) up to size {}", cli.max_n);
    let polys = generate_polys_up_to_size(cli.max_n);

    if cli.report_polys {
        report_polys(cli, polys);
    }
}

fn generate_polys_up_to_size(max_n: usize) -> HashMap<usize, HashSet<ShapeWithGrid>> {
    let mut known_polys: HashMap<usize, HashSet<ShapeWithGrid>> = HashMap::new();
    for n in 1..=max_n {
        let polys = generate_polys_of_size(n, &known_polys);
        known_polys.entry(n).or_insert(polys);
    }
    known_polys
}

fn generate_polys_of_size(
    n: usize,
    known_polys: &HashMap<usize, HashSet<ShapeWithGrid>>,
) -> HashSet<ShapeWithGrid> {
    let start = Instant::now();
    print!("size: {: >2}... ", n);

    if n == 1 {
        report_performance(start, 1, 1, 1);
        return HashSet::from([ShapeWithGrid::canonical(vec![Vector2::new(0, 0)])]);
    }

    let prev_polys: &HashSet<ShapeWithGrid> = &known_polys[&(n - 1)];
    let result: (usize, usize, HashSet<ShapeWithGrid>) = prev_polys
        .par_iter()
        .fold(
            || (0, 0, HashSet::<ShapeWithGrid>::new()),
            |(mut points_tried, mut polys_tried, mut new_polys), prev_poly| {
                for p in &prev_poly.points {
                    for m in MOVES {
                        points_tried += 1;
                        let new_point = p + m;
                        if prev_poly.points.contains(&new_point) {
                            continue;
                        }

                        polys_tried += 1;
                        // cloning then pushing would force an unnecessary grow, so we initialize with the correct size
                        let mut new_points = Vec::with_capacity(prev_poly.points.len() + 1);
                        new_points.extend(&prev_poly.points);
                        new_points.push(new_point);

                        let new_poly = ShapeWithGrid::canonical(new_points);
                        new_polys.insert(new_poly);
                    }
                }
                (points_tried, polys_tried, new_polys)
            },
        )
        .reduce(
            || (0, 0, HashSet::<ShapeWithGrid>::new()),
            |mut a, b| {
                a.2.extend(b.2);
                (a.0 + b.0, a.1 + b.1, a.2)
            },
        );

    let new_polys = result.2;
    report_performance(start, result.0, result.1, new_polys.len());
    new_polys
}

fn report_performance(start: Instant, points_tried: usize, polys_tried: usize, found: usize) {
    let dur = start.elapsed();

    let points_tried_string = format!(
        "points tried: {: >10} {: >12}",
        points_tried,
        format!(
            "({}/s)",
            NUM.format(".3s", points_tried as f64 / dur.as_secs_f64())
        )
    );

    let polys_tried_string = format!(
        "polys tried: {: >10} {: >12}",
        polys_tried,
        format!(
            "({}/s)",
            NUM.format(".3s", polys_tried as f64 / dur.as_secs_f64())
        )
    );

    let found_string = format!(
        "found: {: >10} {: >12}",
        found,
        format!(
            "({}/s)",
            NUM.format(".3s", found as f64 / dur.as_secs_f64())
        )
    );

    println!(
        "time: {}s {: <40} {: <40} {: <40}",
        dur.as_secs(),
        points_tried_string,
        polys_tried_string,
        found_string
    );
}

fn report_polys(cli: Poly2d, known_polys: HashMap<usize, HashSet<ShapeWithGrid>>) {
    for n in 1..=cli.max_n {
        println!("Polys with size n={}", n);
        for poly in &known_polys[&n] {
            println!("{}", &poly);
        }
    }
}
