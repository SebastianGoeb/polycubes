use std::cmp::min;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use nalgebra::{Rotation2, Vector2};

#[derive(Debug, PartialEq, Eq)]
pub struct ShapeMinimal {
    pub points: Vec<Vector2<i8>>,
    bounds: Vector2<i8>,
}

impl ShapeMinimal {
    pub fn new(points: Vec<Vector2<i8>>) -> Self {
        let min: Vector2<i8> = Vector2::new(
            points.iter().map(|p| p.x).min().unwrap(),
            points.iter().map(|p| p.y).min().unwrap(),
        );
        let points = if min != Vector2::new(0, 0) {
            points.iter().map(|p| p - min).collect_vec()
        } else {
            points
        };
        let bounds = Vector2::new(
            points.iter().map(|p| p.x).max().unwrap(),
            points.iter().map(|p| p.y).max().unwrap(),
        );
        ShapeMinimal { points, bounds }
    }

    // assumes points are already aligned with the origin
    // (no points are negative, and some points touch both axes)
    pub fn canonical_clone_with_grid(&self, rotations: &[Rotation2<i8>]) -> ShapeMinimal {
        let (rotation, realign_offset, bounds, _) = rotations.iter()
            .map(|rotation| {
                // calculate how to offset the shape post-rotation
                // such that it's aligned with the origin again
                let rotated_bounds: Vector2<i8> = rotation * self.bounds;
                let realign_offset: Vector2<i8> = Vector2::new(
                    -min(rotated_bounds.x, 0),
                    -min(rotated_bounds.y, 0),
                );
                let realigned_bounds = rotated_bounds.abs();

                // create 1-hot grid by setting each bit to 1 where there is a point
                let mut grid = vec![0; realigned_bounds.y as usize + 1];
                for point in &self.points {
                    let point_rotated = rotation * point + realign_offset;
                    grid[point_rotated.y as usize] |= 0x1 << point_rotated.x
                }

                (rotation, realign_offset, realigned_bounds, grid)
            })
            .min_by(|(_, _, _, grid1), (_, _, _, grid2)| grid1.cmp(grid2))
            .unwrap();

        let points = self.points.iter()
            .map(|p| rotation * p + realign_offset)
            .collect_vec();

        ShapeMinimal { points, bounds }
    }
}

impl Hash for ShapeMinimal {
    fn hash<H>(&self, state: &mut H) where H: Hasher,
    {
        self.points.hash(state);
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Vector2;

    use crate::poly_2d::rotation::ROTATIONS8;
    use crate::poly_2d::shape::shape_minimal::ShapeMinimal;

    //  xxx
    // xx
    fn rot0() -> ShapeMinimal {
        ShapeMinimal {
            points: vec![
                Vector2::new(0, 0),
                Vector2::new(1, 0),
                Vector2::new(1, 1),
                Vector2::new(2, 1),
                Vector2::new(3, 1),
            ],
            bounds: Vector2::new(3, 1),
        }
    }

    // x
    // x
    // xx
    //  x
    fn rot90() -> ShapeMinimal {
        ShapeMinimal {
            points: vec![
                Vector2::new(1, 0),
                Vector2::new(1, 1),
                Vector2::new(0, 1),
                Vector2::new(0, 2),
                Vector2::new(0, 3),
            ],
            bounds: Vector2::new(1, 3),
        }
    }

    //   xx
    // xxx
    fn rot180() -> ShapeMinimal {
        ShapeMinimal {
            points: vec![
                Vector2::new(3, 1),
                Vector2::new(2, 1),
                Vector2::new(2, 0),
                Vector2::new(1, 0),
                Vector2::new(0, 0),
            ],
            bounds: Vector2::new(3, 1),
        }
    }

    // x
    // xx
    //  x
    //  x
    fn rot270() -> ShapeMinimal {
        ShapeMinimal {
            points: vec![
                Vector2::new(0, 3),
                Vector2::new(0, 2),
                Vector2::new(1, 2),
                Vector2::new(1, 1),
                Vector2::new(1, 0),
            ],
            bounds: Vector2::new(1, 3),
        }
    }

    #[test]
    fn should_canonicalize() {
        for shape in [
            rot0(),
            rot90(),
            rot180(),
            rot270()
        ] {
            assert_eq!(
                shape.canonical_clone_with_grid(ROTATIONS8),
                rot270()
            );
        }
    }
}