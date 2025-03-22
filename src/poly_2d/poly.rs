use lazy_static::lazy_static;
use log::{debug, info, log_enabled};
use nalgebra::{SVector, Vector2};
use num::{Integer, Zero};
use rayon::prelude::*;
use std::fmt::Debug;
use std::ops::AddAssign;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::cli::{Algorithm, Poly2d};
use crate::poly_2d::shape::shape_generic::ShapeN;
use crate::poly_2d::shape::shape_minimal::ShapeMinimal;
use crate::poly_2d::shape::shape_with_grid::ShapeWithGrid;

lazy_static! {
    static ref NUM: format_num::NumberFormat = format_num::NumberFormat::new();
}

pub fn generate_polys(cli: Poly2d) {
    let alg = cli.algorithm.clone().unwrap_or(Algorithm::A32);

    info!(
        "generating polycubes (in 2d) up to size {} with algorithm {}",
        cli.max_n, alg
    );

    match alg {
        Algorithm::A32 => {
            let polys = generate_shapes_up_to_size::<ShapeWithGrid, i32>(cli.max_n);
            if cli.report_polys {
                report_polys(cli, polys);
            }
        }
        Algorithm::B8 => {
            generate_shapes_up_to_size::<ShapeMinimal, i8>(cli.max_n);
        }
    }
}

fn generate_shapes_up_to_size<S, T>(max_n: usize) -> HashMap<usize, HashSet<S>>
where
    S: ShapeN<T, 2>,
    T: Integer + Zero + Clone + Debug + AddAssign + Send + Sync + 'static,
{
    let mut known_polys: HashMap<usize, HashSet<S>> = HashMap::new();
    for n in 1..=max_n {
        let polys = generate_shapes_with_size(n, &known_polys);
        if log_enabled!(log::Level::Debug) {
            for p in &polys {
                debug!("{:?}", p)
            }
        }
        known_polys.entry(n).or_insert(polys);
    }
    known_polys
}

fn generate_shapes_with_size<S, T>(n: usize, known_polys: &HashMap<usize, HashSet<S>>) -> HashSet<S>
where
    S: ShapeN<T, 2>,
    T: Integer + Zero + Clone + Debug + AddAssign + Send + Sync + 'static,
{
    let start = Instant::now();

    let moves = S::moves();

    if n == 1 {
        report_performance(n, start, 1, 1, 1);
        return HashSet::from([S::new(vec![Vector2::new(T::zero(), T::zero())])]);
    }

    let prev_polys: &HashSet<S> = &known_polys[&(n - 1)];
    let result: (usize, usize, HashSet<S>) = prev_polys
        .par_iter()
        .fold(
            || (0, 0, HashSet::<S>::new()),
            |(mut points_tried, mut polys_tried, mut new_polys), prev_poly| {
                let points = prev_poly.points();
                for p in points {
                    for m in moves {
                        points_tried += 1;
                        let new_point = p + m;
                        if points.contains(&new_point) {
                            continue;
                        }

                        polys_tried += 1;
                        // cloning then pushing would force an unnecessary grow, so we initialize with the correct size
                        let mut new_points: Vec<SVector<T, 2>> =
                            Vec::with_capacity(points.len() + 1);
                        new_points.extend_from_slice(points);
                        new_points.push(new_point);

                        let new_poly = S::new(new_points);
                        new_polys.insert(new_poly);
                    }
                }
                (points_tried, polys_tried, new_polys)
            },
        )
        .reduce(
            || (0, 0, HashSet::<S>::new()),
            |mut a, b| {
                a.2.extend(b.2);
                (a.0 + b.0, a.1 + b.1, a.2)
            },
        );

    let new_polys = result.2;
    report_performance(n, start, result.0, result.1, new_polys.len());
    new_polys
}

fn report_performance(
    size: usize,
    start: Instant,
    points_tried: usize,
    polys_tried: usize,
    found: usize,
) {
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

    info!(
        "size: {: >2}    time: {}s    {: <40} {: <40} {: <40}",
        size,
        dur.as_secs(),
        points_tried_string,
        polys_tried_string,
        found_string
    );
}

fn report_polys(cli: Poly2d, known_polys: HashMap<usize, HashSet<ShapeWithGrid>>) {
    for n in 1..=cli.max_n {
        info!("Polys with size n={}", n);
        for poly in &known_polys[&n] {
            info!("{}", &poly);
        }
    }
}
