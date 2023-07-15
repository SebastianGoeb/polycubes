use std::cmp::{max, min};

use ndarray::*;
use rand::{seq::SliceRandom, thread_rng};

pub fn generate_polycubes_naive() {
    generate_polycubes(9)
}

fn generate_polycubes(size: usize) {
    let shape = grow_random_shape(2, size);
    println!("{:#?}", shape.crop());
}

#[derive(Debug, Clone)]
struct BoundingBox {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug)]
struct Shape {
    grid: Array2<u8>,
    bounds: BoundingBox,
}

impl Shape {
    fn crop(&self) -> ArrayView2<u8> {
        let b = self.bounds.clone();
        self.grid.slice(s![b.min_x..=b.max_x, b.min_y..=b.max_y])
    }
}

fn grow_random_shape(_dim: usize, size: usize) -> Shape {
    // allow enough space to grow linearly in any direction
    let grid_size = size * 2 - 1;
    let mut grid = Array2::<u8>::zeros((grid_size, grid_size));

    // start at the center of the grid
    let mut location = (size - 1, size - 1);
    grid[location] = 1;

    let mut bounds = BoundingBox {
        min_x: location.0,
        max_x: location.0,
        min_y: location.1,
        max_y: location.1,
    };

    for i in 2..=size {
        // decide random direction
        // TODO optimize constant shuffling
        let mut directions: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        let new_location: Option<(usize, usize)> = directions
            .iter()
            .map(|direction| {
                (
                    (location.0 as isize + direction.0) as usize,
                    (location.1 as isize + direction.1) as usize,
                )
            })
            .find(|new_location| grid[new_location.clone()] == 0);

        location = new_location.expect("no available moves");

        // grow in that direction
        grid[location] = i as u8;

        // update bounds
        bounds.min_x = min(bounds.min_x, location.0);
        bounds.max_x = max(bounds.max_x, location.0);
        bounds.min_y = min(bounds.min_y, location.1);
        bounds.max_y = max(bounds.max_y, location.1);
    }

    return Shape { grid, bounds };
}
