use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use rayon::prelude::*;

use crate::cli::{Algorithm, Poly2d};
use crate::poly_2d::moves::{MOVES32, MOVES8};
use crate::poly_2d::shape::shape_minimal::ShapeMinimal;
use crate::poly_2d::shape::shape_with_grid::ShapeWithGrid;

lazy_static! {
    static ref NUM: format_num::NumberFormat = format_num::NumberFormat::new();
}

pub fn generate_polys(cli: Poly2d) {
    let alg = cli.algorithm.clone().unwrap_or(Algorithm::A32);

    println!("generating polycubes (in 2d) up to size {} with algorithm {}", cli.max_n, alg);

    match alg {
        Algorithm::A32 => {
            let polys = generate_shape_with_grid_up_to(cli.max_n);
            if cli.report_polys {
                report_polys(cli, polys);
            }
        }
        Algorithm::B8 => {
            let polys = generate_shape_minimal_up_to(cli.max_n);
            if cli.report_polys {
                for (size, polys_level) in polys.iter()
                    .sorted_by(|a, b| a.0.cmp(b.0)) {
                    println!("\nsize: {}", size);
                    for p in polys_level {
                        println!("{:?}", p);
                    }
                }
            }
        }
    }
}

fn generate_shape_with_grid_up_to(max_n: usize) -> HashMap<usize, HashSet<ShapeWithGrid>> {
    let mut known_polys: HashMap<usize, HashSet<ShapeWithGrid>> = HashMap::new();
    for n in 1..=max_n {
        let polys = generate_shape_with_grid_level(n, &known_polys);
        known_polys.entry(n).or_insert(polys);
    }
    known_polys
}

fn generate_shape_with_grid_level(
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
                    for m in MOVES32 {
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

fn generate_shape_minimal_up_to(max_n: usize) -> HashMap<usize, HashSet<ShapeMinimal>> {
    let mut known_polys: HashMap<usize, HashSet<ShapeMinimal>> = HashMap::new();
    for n in 1..=max_n {
        let polys = generate_shape_minimal_level(n, &known_polys);
        known_polys.entry(n).or_insert(polys);
    }
    known_polys
}

fn generate_shape_minimal_level(
    n: usize,
    known_polys: &HashMap<usize, HashSet<ShapeMinimal>>,
) -> HashSet<ShapeMinimal> {
    let start = Instant::now();
    print!("size: {: >2}... ", n);

    if n == 1 {
        report_performance(start, 1, 1, 1);
        return HashSet::from([ShapeMinimal::new(vec![Vector2::new(0, 0)])]);
    }

    let prev_polys: &HashSet<ShapeMinimal> = &known_polys[&(n - 1)];
    let result: (usize, usize, HashSet<ShapeMinimal>) = prev_polys
        .par_iter()
        .fold(
            || (0, 0, HashSet::<ShapeMinimal>::new()),
            |(mut points_tried, mut polys_tried, mut new_polys), prev_poly| {
                for p in &prev_poly.points {
                    for m in MOVES8 {
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

                        let new_poly = ShapeMinimal::new(new_points);
                        new_polys.insert(new_poly);
                    }
                }
                (points_tried, polys_tried, new_polys)
            },
        )
        .reduce(
            || (0, 0, HashSet::<ShapeMinimal>::new()),
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
