use std::fmt::Display;
use std::hash::{Hash, Hasher};

use nalgebra::{Rotation2, SVector, Vector2};

use crate::poly_2d::rotation::ROTATIONS32;
use crate::poly_2d::shape::bounding_box_two_points::BoundingBoxTwoPoints;
use crate::poly_2d::shape::shape_generic::ShapeN;

#[derive(Debug, Eq)]
pub struct ShapeWithGrid {
    pub points: Vec<Vector2<i32>>,
    pub grid_bounds: BoundingBoxTwoPoints,
    pub grid: Vec<u64>,
}

impl ShapeN<i32, 2> for ShapeWithGrid {
    fn new(points: Vec<Vector2<i32>>) -> Self {
        // TODO cache and extend bounds instead of always recomputing
        let bounds = BoundingBoxTwoPoints::from(&points);

        let mut best: Option<(BoundingBoxTwoPoints, Vec<u64>)> = None;
        for rotation in ROTATIONS32 {
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
        ShapeWithGrid {
            points,
            grid_bounds: best.0,
            grid: best.1,
        }
    }

    fn points(&self) -> &[SVector<i32, 2>] {
        &self.points
    }
}

fn rotate_shape(
    points: &Vec<Vector2<i32>>,
    bounds: &BoundingBoxTwoPoints,
    rotation: &Rotation2<i32>,
) -> (BoundingBoxTwoPoints, Vec<u64>) {
    let bounds_rotated = rotation * bounds;
    let bounds_rotated_min = bounds_rotated.min();
    let bounds_rotated_normalized = bounds_rotated - bounds_rotated_min;
    let bounds_rotated_normalized_max = bounds_rotated_normalized.max();

    let mut grid = vec![0; bounds_rotated_normalized_max.y as usize + 1];
    for p in points {
        // normalize points to be >= 0 in all axes
        let p = rotation * p - bounds_rotated_min;
        // Row major order, so each row/u64 extends in the x direction. They are indexed in the y direction.
        grid[p.y as usize] |= 0x1 << p.x
    }

    (bounds_rotated_normalized, grid)
}

impl PartialEq for ShapeWithGrid {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Hash for ShapeWithGrid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl Display for ShapeWithGrid {
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
