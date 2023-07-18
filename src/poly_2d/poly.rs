use std::{cell::OnceCell, collections::HashMap, fmt::Display, time::Instant};

use ndarray::{Array2, Axis};

use crate::cli::Poly2d;

pub fn generate_polys(cli: Poly2d) {
    print!("generating polys up to size {}", cli.max_n);

    let mut known_polys: HashMap<usize, Vec<Shape>> = HashMap::new();
    for n in 1..=cli.max_n {
        let polys = generate_polys_of_size(n, &known_polys);
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

fn generate_polys_of_size(n: usize, known_polys: &HashMap<usize, Vec<Shape>>) -> Vec<Shape> {
    let start = Instant::now();
    print!("size: {: >2}... ", n);

    if n == 1 {
        report_performance(start, 1, 1);
        return vec![Shape::from(vec![(0, 0)])];
    }

    let prev_polys: &Vec<Shape> = &known_polys[&(n - 1)];
    let mut new_polys: Vec<Shape> = Vec::with_capacity(prev_polys.len() * 5);
    let mut tried = 0;
    for prev_poly in prev_polys {
        for prev_point in &prev_poly.points {
            for m in &MOVES {
                tried += 1;
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

    report_performance(start, tried, new_polys.len());
    new_polys
}

fn is_duplicate(polys: &Vec<Shape>, poly: &Shape) -> bool {
    return polys.iter().any(|other| {
        if other.dimensions() != poly.dimensions() {
            return false;
        }

        if other.degrees() != poly.degrees() {
            return false;
        }

        let mut rot90 = other.grid().t();
        rot90.invert_axis(Axis(1));

        let mut rot180 = other.grid().view();
        rot180.invert_axis(Axis(0));
        rot180.invert_axis(Axis(1));

        let mut rot270 = other.grid().t();
        rot270.invert_axis(Axis(0));

        other.grid() == poly.grid()
            || rot90 == poly.grid()
            || rot180 == poly.grid()
            || rot270 == poly.grid()
    });
}

fn report_performance(start: Instant, tried: usize, found: usize) {
    let dur = start.elapsed();

    let tried_string = format!(
        "tried: {} ({:.0}/s)",
        tried,
        tried as f64 * 1000.0 / dur.as_millis() as f64
    );

    let found_string = format!(
        "found: {} ({:.0}/s)",
        found,
        found as f64 * 1000.0 / dur.as_millis() as f64
    );

    println!(
        "time: {}s {: <25} {: <25}",
        dur.as_secs(),
        tried_string,
        found_string
    );
}

struct Shape {
    points: Vec<(i32, i32)>,
    grid: OnceCell<Array2<i32>>,
    dimensions: OnceCell<Vec<usize>>,
    degrees: OnceCell<Vec<usize>>,
}

impl Shape {
    fn from(points: Vec<(i32, i32)>) -> Shape {
        Shape {
            points,
            grid: OnceCell::new(),
            dimensions: OnceCell::new(),
            degrees: OnceCell::new(),
        }
    }

    // TODO i32 overkill, can be bool or u8?
    fn grid(&self) -> &Array2<i32> {
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

    fn dimensions(&self) -> &Vec<usize> {
        return self.dimensions.get_or_init(|| {
            let mut dims = self.grid().shape().to_vec();
            dims.sort();
            dims
        });
    }

    fn degrees(&self) -> &Vec<usize> {
        return self.degrees.get_or_init(|| {
            let grid = self.grid();
            let w = grid.len_of(Axis(0));
            let h = grid.len_of(Axis(1));

            let mut degrees: Vec<usize> = vec![0, 0, 0, 0, 0]; // degrees 0 to 4
            for x in 0..w {
                for y in 0..h {
                    if grid[[x, y]] != 0 {
                        let mut degree = 0;
                        if x > 0 && grid[[x - 1, y]] != 0 {
                            degree += 1;
                        }
                        if x < w - 1 && grid[[x + 1, y]] != 0 {
                            degree += 1;
                        }
                        if y > 0 && grid[[x, y - 1]] != 0 {
                            degree += 1;
                        }
                        if y < h - 1 && grid[[x, y + 1]] != 0 {
                            degree += 1;
                        }
                        degrees[degree] += 1;
                    }
                }
            }

            degrees
        });
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.grid() == other.grid()
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
