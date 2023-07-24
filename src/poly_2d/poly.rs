use std::cmp;
use std::hash::{Hash, Hasher};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Instant,
};

use itertools::Itertools;
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

impl std::ops::Mul<BoundingBox> for &Rotation2<i32> {
    type Output = BoundingBox;

    fn mul(self, rhs: BoundingBox) -> Self::Output {
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
    bounds: BoundingBox,
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
        let (points, bounds, grid) = ROTATIONS
            .iter()
            // calculate all possible rotations
            .map(|rotation| {
                let (points, bounds) = rotate_and_normalize_points(&points, rotation);
                // bounds.min() should be (0, 0) at this point, and max() should be positive
                let grid = points_to_grid(&points, &bounds);
                (points, bounds, grid)
            })
            // select rotation that gives the lowest overall binary grid
            .min_by(|(_, _, grida), (_, _, gridb)| grida.cmp(gridb))
            .unwrap();
        BinShape {
            points,
            bounds,
            grid,
        }
    }
}

fn rotate_and_normalize_points(
    points: &[Vector2<i32>],
    rotation: &Rotation2<i32>,
) -> (Vec<Vector2<i32>>, BoundingBox) {
    let bounds = BoundingBox::from(points);
    let bounds_rotated = rotation * bounds;
    let bounds_rotated_min = bounds_rotated.min();

    let points_rotated_normalized = points
        .iter()
        .map(|p| rotation * p)
        .map(|p| p - bounds_rotated_min) // normalize points to be >= 0 in all axes
        .collect_vec();
    let bounds_rotated_normalized = bounds_rotated - bounds_rotated_min;

    (points_rotated_normalized, bounds_rotated_normalized)
}

fn points_to_grid(points: &Vec<Vector2<i32>>, bounds: &BoundingBox) -> Vec<u64> {
    // Row major order, so each row/u64 extends in the x direction. They are indexed in the y direction.
    let mut grid = vec![0; bounds.max().y as usize + 1];
    for p in points {
        grid[p.y as usize] |= 0x1 << p.x
    }
    grid
}

impl Display for BinShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for i_x in 0..self.bounds.max().x + 1 {
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
