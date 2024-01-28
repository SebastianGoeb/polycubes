use std::fmt::Display;
use std::hash::{Hash, Hasher};

use nalgebra::Vector2;

use crate::poly_2d::shape::bounding_box_two_points::BoundingBoxTwoPoints;

#[derive(Debug, Eq)]
pub struct ShapeWithGrid {
    pub points: Vec<Vector2<i32>>,
    pub grid_bounds: BoundingBoxTwoPoints,
    pub grid: Vec<u64>,
}

impl PartialEq for ShapeWithGrid {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Hash for ShapeWithGrid {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
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

