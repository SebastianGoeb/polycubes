use std::cmp;
use std::hash::{Hash, Hasher};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Instant,
};

use nalgebra::Matrix2;
use nalgebra::{Rotation2, Vector2};

use crate::cli::Poly2d;

static MOVES: &[Vector2<i32>] = &[
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
];

static ROTATIONS: &[Rotation2<i32>] = &[
    Rotation2::from_matrix_unchecked(Matrix2::new(1, 0, 0, 1)), // 0 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, -1, 1, 0)), // 90 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(-1, 0, 0, -1)), // 180 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, 1, -1, 0)), // 270 deg ccw
];

pub fn generate_polys(cli: Poly2d) {
    println!("generating polycubes (in 2d) up to size {}", cli.max_n);
    let polys = generate_polys_up_to_size(cli.max_n);

    if cli.report_polys {
        report_polys(cli, polys);
    }
}

fn generate_polys_up_to_size(max_n: usize) -> HashMap<usize, HashSet<BinShape>> {
    let mut known_polys: HashMap<usize, HashSet<BinShape>> = HashMap::new();
    for n in 1..=max_n {
        let polys = generate_polys_of_size(n, &known_polys);
        known_polys.entry(n).or_insert(polys);
    }
    known_polys
}

fn generate_polys_of_size(
    n: usize,
    known_polys: &HashMap<usize, HashSet<BinShape>>,
) -> HashSet<BinShape> {
    let start = Instant::now();
    print!("size: {: >2}... ", n);

    if n == 1 {
        report_performance(start, 1, 1);
        return HashSet::from([BinShape::canonical(vec![Vector2::new(0, 0)])]);
    }

    let prev_polys: &HashSet<BinShape> = &known_polys[&(n - 1)];
    // each generation seems to be ~4x as large as the previous one, so we allocate some extra space to avoid growing.
    let mut new_polys: HashSet<BinShape> = HashSet::with_capacity(prev_polys.len() * 5);
    let mut tried = 0;
    for prev_poly in prev_polys {
        let mut possible_points: HashSet<Vector2<i32>> =
            HashSet::with_capacity(prev_poly.points.len() * MOVES.len());
        for p in &prev_poly.points {
            for m in MOVES {
                let new_point = p + m;
                if !prev_poly.points.contains(&new_point) {
                    possible_points.insert(new_point);
                }
            }
        }

        tried += possible_points.len();
        for new_point in possible_points {
            // cloning then pushing would force an unnecessary grow, so we initialize with the correct size
            let mut new_points = Vec::with_capacity(prev_poly.points.len() + 1);
            new_points.extend(&prev_poly.points);
            new_points.push(new_point);

            let new_poly = BinShape::canonical(new_points);
            new_polys.insert(new_poly);
        }
    }

    report_performance(start, tried, new_polys.len());
    new_polys
}

#[derive(PartialEq, Eq, Debug)]
struct BoundingBox {
    p0: Vector2<i32>,
    p1: Vector2<i32>,
}

impl BoundingBox {
    fn from(points: &[Vector2<i32>]) -> BoundingBox {
        return BoundingBox {
            p0: Vector2::new(
                points.iter().map(|p| p.x).min().unwrap(),
                points.iter().map(|p| p.y).min().unwrap(),
            ),
            p1: Vector2::new(
                points.iter().map(|p| p.x).max().unwrap(),
                points.iter().map(|p| p.y).max().unwrap(),
            ),
        };
    }

    fn min(&self) -> Vector2<i32> {
        Vector2::new(
            cmp::min(self.p0.x, self.p1.x),
            cmp::min(self.p0.y, self.p1.y),
        )
    }

    fn max(&self) -> Vector2<i32> {
        Vector2::new(
            cmp::max(self.p0.x, self.p1.x),
            cmp::max(self.p0.y, self.p1.y),
        )
    }
}

impl std::ops::Mul<&BoundingBox> for &Rotation2<i32> {
    type Output = BoundingBox;

    fn mul(self, rhs: &BoundingBox) -> Self::Output {
        BoundingBox {
            p0: self * rhs.p0,
            p1: self * rhs.p1,
        }
    }
}

impl std::ops::Add<Vector2<i32>> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Vector2<i32>) -> BoundingBox {
        BoundingBox {
            p0: self.p0 + rhs,
            p1: self.p1 + rhs,
        }
    }
}

impl std::ops::Sub<Vector2<i32>> for BoundingBox {
    type Output = BoundingBox;

    fn sub(self, rhs: Vector2<i32>) -> BoundingBox {
        BoundingBox {
            p0: self.p0 - rhs,
            p1: self.p1 - rhs,
        }
    }
}

#[derive(Eq)]
struct BinShape {
    points: Vec<Vector2<i32>>,
    grid_bounds: BoundingBox,
    grid: Vec<u64>,
}

impl PartialEq for BinShape {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Hash for BinShape {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.grid.hash(state);
    }
}

impl BinShape {
    fn canonical(points: Vec<Vector2<i32>>) -> BinShape {
        // TODO cache and extend bounds instead of always recomputing
        let bounds = BoundingBox::from(&points);

        let mut best: Option<(BoundingBox, Vec<u64>)> = None;
        for rotation in ROTATIONS {
            let candidate = rotate_shape(&points, &bounds, rotation);

            match &best {
                Some(b) => {
                    if candidate.1 < b.1 {
                        best = Some(candidate)
                    }
                }
                None => best = Some(candidate),
            }
        }

        let best = best.unwrap();
        BinShape {
            points,
            grid_bounds: best.0,
            grid: best.1,
        }
    }
}

fn rotate_shape(
    points: &Vec<Vector2<i32>>,
    bounds: &BoundingBox,
    rotation: &Rotation2<i32>,
) -> (BoundingBox, Vec<u64>) {
    let bounds_rotated = rotation * bounds;
    let bounds_rotated_min = bounds_rotated.min();
    let bounds_rotated_normalized = bounds_rotated - bounds_rotated_min;
    let bounds_rotated_normalized_max = bounds_rotated_normalized.max();

    let mut grid = vec![0; bounds_rotated_normalized_max.y as usize + 1];
    points
        .iter()
        .map(|p| rotation * p - bounds_rotated_min) // normalize points to be >= 0 in all axes
        // Row major order, so each row/u64 extends in the x direction. They are indexed in the y direction.
        .for_each(|p| grid[p.y as usize] |= 0x1 << p.x);
    (bounds_rotated_normalized, grid)
}

impl Display for BinShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for i_x in 0..self.grid_bounds.max().x + 1 {
                let present = (row >> i_x) & 0x1 != 0;
                write!(f, "{}", if present { 'O' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
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

fn report_polys(cli: Poly2d, known_polys: HashMap<usize, HashSet<BinShape>>) {
    for n in 1..=cli.max_n {
        println!("Polys with size n={}", n);
        for poly in &known_polys[&n] {
            println!("{}", &poly);
        }
    }
}
