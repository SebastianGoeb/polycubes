use std::hash::{Hash, Hasher};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Instant,
};

use itertools::Itertools;

use crate::cli::Poly2d;

static MOVES: [&(i32, i32); 4] = [&(0, 1), &(0, -1), &(1, 0), &(-1, 0)];

pub fn generate_polys(cli: Poly2d) {
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
    return known_polys;
}

fn generate_polys_of_size(
    n: usize,
    known_polys: &HashMap<usize, HashSet<BinShape>>,
) -> HashSet<BinShape> {
    let start = Instant::now();
    print!("size: {: >2}... ", n);

    if n == 1 {
        report_performance(start, 1, 1);
        return HashSet::from([BinShape::canonical(vec![(0, 0)])]);
    }

    let prev_polys: &HashSet<BinShape> = &known_polys[&(n - 1)];
    // each generation seems to be ~4x as large as the previous one, so we allocate some extra space to avoid growing.
    let mut new_polys: HashSet<BinShape> = HashSet::with_capacity(prev_polys.len() * 5);
    let mut tried = 0;
    for prev_poly in prev_polys {
        let possible_points: Vec<(i32, i32)> = prev_poly
            .points
            .iter()
            .flat_map(|p| MOVES.iter().map(|m| (p.0 + m.0, p.1 + m.1)))
            .unique()
            .filter(|p| !prev_poly.points.contains(p))
            .collect_vec();

        tried += possible_points.len();
        for new_point in possible_points {
            let mut new_points = prev_poly.points.clone();
            new_points.push(new_point);

            let new_poly = BinShape::canonical(new_points);
            new_polys.insert(new_poly);
        }
    }

    report_performance(start, tried, new_polys.len());
    new_polys
}

#[derive(Eq)]
struct BinShape {
    points: Vec<(i32, i32)>,
    grid: Vec<u64>,
    dimensions: (usize, usize),
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
    fn canonical(points: Vec<(i32, i32)>) -> BinShape {
        let min_0: i32 = points.iter().map(|p| p.0).min().unwrap();
        let max_0: i32 = points.iter().map(|p| p.0).max().unwrap();
        let min_1: i32 = points.iter().map(|p| p.1).min().unwrap();
        let max_1: i32 = points.iter().map(|p| p.1).max().unwrap();
        let dim_0 = (max_0 - min_0 + 1) as usize;
        let dim_1 = (max_1 - min_1 + 1) as usize;

        let dims_0 = (dim_0, dim_1);
        let mut grid_0 = vec![0; dim_0];
        points
            .iter()
            .map(|p| p.translate((-min_0, -min_1)))
            .for_each(|p| grid_0[p.0 as usize] |= 0x1 << p.1);

        let dims_90 = (dim_1, dim_0);
        let mut grid_90 = vec![0; dim_1];
        points
            .iter()
            .map(|p| p.rotate90())
            .map(|p| p.translate((max_1, -min_0)))
            .for_each(|p| grid_90[p.0 as usize] |= 0x1 << p.1);

        let dims_180 = (dim_0, dim_1);
        let mut grid_180 = vec![0; dim_0];
        points
            .iter()
            .map(|p| p.rotate180())
            .map(|p| p.translate((max_0, max_1)))
            .for_each(|p| grid_180[p.0 as usize] |= 0x1 << p.1);

        let dims_270 = (dim_1, dim_0);
        let mut grid_270 = vec![0; dim_1];
        points
            .iter()
            .map(|p| p.rotate270())
            .map(|p| p.translate((-min_1, max_0)))
            .for_each(|p| grid_270[p.0 as usize] |= 0x1 << p.1);

        let idx: usize = vec![&grid_0, &grid_90, &grid_180, &grid_270]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .0;

        match idx {
            0 => BinShape {
                points,
                grid: grid_0,
                dimensions: dims_0,
            },
            1 => BinShape {
                points,
                grid: grid_90,
                dimensions: dims_90,
            },
            2 => BinShape {
                points,
                grid: grid_180,
                dimensions: dims_180,
            },
            3 => BinShape {
                points,
                grid: grid_270,
                dimensions: dims_270,
            },
            _ => panic!(),
        }
    }
}

impl Display for BinShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for i_1 in 0..self.dimensions.1 {
                let present = (row >> i_1) & 0x1 != 0;
                write!(f, "{}", if present { 'O' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

trait Vec2 {
    fn rotate90(&self) -> Self;
    fn rotate180(&self) -> Self;
    fn rotate270(&self) -> Self;
    fn translate(&self, t: (i32, i32)) -> Self;
}

impl Vec2 for (i32, i32) {
    fn rotate90(&self) -> Self {
        (-self.1, self.0)
    }
    fn rotate180(&self) -> Self {
        (-self.0, -self.1)
    }
    fn rotate270(&self) -> Self {
        (self.1, -self.0)
    }

    fn translate(&self, t: (i32, i32)) -> Self {
        (self.0 + t.0, self.1 + t.1)
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
