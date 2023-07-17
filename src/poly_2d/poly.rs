use std::{cell::OnceCell, collections::HashMap, fmt::Display, time::Instant};

use itertools::Itertools;
use ndarray::{Array2, Axis};

use crate::cli::Poly2d;

pub fn generate_polys_2d(cli: Poly2d) {
    let mut known_polys: HashMap<usize, Vec<Shape>> = HashMap::new();
    for n in 1..=cli.max_n {
        let polys = generate_generation(n, &known_polys);
        known_polys.entry(n).or_insert(polys);
    }

    if cli.report_polys {
        for n in 1..=cli.max_n {
            println!("Polys with size n={}", n);
            for poly in &known_polys[&n] {
                println!("{}", poly);
            }
        }
    }
}

static MOVES: [&(i32, i32); 4] = [&(0, 1), &(0, -1), &(1, 0), &(-1, 0)];

fn generate_generation(n: usize, known_polys: &HashMap<usize, Vec<Shape>>) -> Vec<Shape> {
    let start = Instant::now();
    println!("generating polys of size {}", n);

    if n == 1 {
        let dur = start.elapsed();
        println!(
            "generated {} polys in {} s ({:.0}/s)\n",
            1,
            dur.as_secs(),
            1000.0 / dur.as_millis() as f64
        );
        return vec![Shape::from(vec![(0, 0)])];
    }

    let prev_polys: &Vec<Shape> = &known_polys[&(n - 1)];
    let mut new_polys: Vec<Shape> = Vec::new();
    for prev_poly in prev_polys {
        for prev_point in &prev_poly.points {
            for m in &MOVES {
                let new_point = (prev_point.0 + m.0, prev_point.1 + m.1);
                if prev_poly.points.contains(&new_point) {
                    // move not allowed
                    continue;
                }

                let mut new_poly = Shape::from(prev_poly.points.clone());
                new_poly.points.push(new_point);

                if is_duplicate(&new_polys, &new_poly) {
                    continue;
                }

                new_polys.push(new_poly);
            }
        }
    }

    let dur = start.elapsed();
    println!(
        "{} polys in {} s ({:.0}/s)\n",
        new_polys.len(),
        dur.as_secs(),
        new_polys.len() as f64 * 1000.0 / dur.as_millis() as f64
    );
    new_polys
}

fn is_duplicate(polys: &Vec<Shape>, poly: &Shape) -> bool {
    return polys.iter().any(|other| {
        let other_shape = other.to_grid().shape().iter().sorted().collect_vec();
        let poly_shape = poly.to_grid().shape().iter().sorted().collect_vec();
        if other_shape != poly_shape {
            return false;
        }

        let mut rot90 = other.to_grid().t();
        rot90.invert_axis(Axis(1));

        let mut rot180 = other.to_grid().view();
        rot180.invert_axis(Axis(0));
        rot180.invert_axis(Axis(1));

        let mut rot270 = other.to_grid().t();
        rot270.invert_axis(Axis(0));

        other.to_grid() == poly.to_grid()
            || rot90 == poly.to_grid()
            || rot180 == poly.to_grid()
            || rot270 == poly.to_grid()
    });
}

struct Shape {
    points: Vec<(i32, i32)>,
    grid: OnceCell<Array2<i32>>,
}

impl Shape {
    fn from(points: Vec<(i32, i32)>) -> Shape {
        Shape {
            points,
            grid: OnceCell::new(),
        }
    }

    // TODO i32 overkill, can be bool or u8?
    fn to_grid(&self) -> &Array2<i32> {
        return self.grid.get_or_init(|| {
            let min_x: i32 = self.points.iter().map(|p| p.0).min().unwrap();
            let max_x: i32 = self.points.iter().map(|p| p.0).max().unwrap();
            let min_y: i32 = self.points.iter().map(|p| p.1).min().unwrap();
            let max_y: i32 = self.points.iter().map(|p| p.1).max().unwrap();
            let w = max_x - min_x + 1;
            let h = max_y - min_y + 1;

            let mut grid = Array2::<i32>::zeros((w as usize, h as usize));
            for p in &self.points {
                grid[((p.0 - min_x) as usize, (p.1 - min_y) as usize)] = 1;
            }

            grid
        });
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.to_grid() == other.to_grid()
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let [] = &self.points[..] {
            write!(f, "This shape has no points")?;
            return Ok(());
        }

        let min_x: i32 = self.points.iter().map(|p| p.0).min().unwrap();
        let max_x: i32 = self.points.iter().map(|p| p.0).max().unwrap();
        let min_y: i32 = self.points.iter().map(|p| p.1).min().unwrap();
        let max_y: i32 = self.points.iter().map(|p| p.1).max().unwrap();
        let w = max_x - min_x + 1;
        let h = max_y - min_y + 1;

        let mut grid = Array2::<i32>::zeros((w as usize, h as usize));
        for (i, p) in self.points.iter().enumerate() {
            grid[((p.0 - min_x) as usize, (p.1 - min_y) as usize)] = i as i32 + 1;
        }
        writeln!(f, "{}", grid)?;

        Ok(())
    }
}