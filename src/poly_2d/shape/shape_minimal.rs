use std::cmp::min;
use std::hash::{Hash, Hasher};

use nalgebra::{Rotation2, Vector2};

use crate::poly_2d::rotation::ROTATIONS8;

fn min_vector2(points: &Vec<Vector2<i8>>) -> Vector2<i8> {
    Vector2::new(
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
    )
}

fn max_vector2(points: &Vec<Vector2<i8>>) -> Vector2<i8> {
    Vector2::new(
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShapeMinimal {
    pub points: Vec<Vector2<i8>>,
}

impl ShapeMinimal {
    pub fn new(mut points: Vec<Vector2<i8>>) -> Self {
        let min: Vector2<i8> = min_vector2(&points);
        let max: Vector2<i8> = max_vector2(&points);
        let (rot, realign_offset) = ShapeMinimal::canonical_rotation(&points, &min, &max);
        for p in &mut points {
            *p = rot * (*p - min) + realign_offset;
        }
        points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
        ShapeMinimal { points }
    }

    fn canonical_rotation(points: &[Vector2<i8>], minp: &Vector2<i8>, maxp: &Vector2<i8>) -> (&'static Rotation2<i8>, Vector2<i8>) {
        let (_, rotation, realign_offset) = ROTATIONS8.iter()
            .map(|rotation| {
                let bounds = maxp - minp;

                // calculate how to offset the shape post-rotation
                // such that it's aligned with the origin again
                let rotated_bounds: Vector2<i8> = rotation * bounds;
                let realign_offset: Vector2<i8> = Vector2::new(
                    -min(rotated_bounds.x, 0),
                    -min(rotated_bounds.y, 0),
                );
                let realigned_bounds = rotated_bounds.abs();

                // create 1-hot grid by setting each bit to 1 where there is a point
                let mut grid: Vec<u64> = vec![0; realigned_bounds.y as usize + 1];
                for point in points {
                    let point_rotated = (rotation * (point - minp)) + realign_offset;
                    grid[point_rotated.y as usize] |= 0x1 << point_rotated.x
                }

                (grid, rotation, realign_offset)
            })
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap();

        (rotation, realign_offset)
    }
}

impl Hash for ShapeMinimal {
    fn hash<H>(&self, state: &mut H) where H: Hasher,
    {
        for p in &self.points {
            p.x.hash(state);
            p.y.hash(state);
        }
        // self.points.hash(state);
    }
}

#[cfg(test)]
mod test {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use itertools::Itertools;
    use nalgebra::Vector2;

    use crate::poly_2d::shape::shape_minimal::ShapeMinimal;

    //  xxx
    // xx
    fn rot0() -> Vec<Vector2<i8>> {
        vec![
            Vector2::new(0, 0),
            Vector2::new(1, 0),
            Vector2::new(1, 1),
            Vector2::new(2, 1),
            Vector2::new(3, 1),
        ]
    }

    // x
    // x
    // xx
    //  x
    fn rot90() -> Vec<Vector2<i8>> {
        vec![
            Vector2::new(1, 0),
            Vector2::new(1, 1),
            Vector2::new(0, 1),
            Vector2::new(0, 2),
            Vector2::new(0, 3),
        ]
    }

    //   xx
    // xxx
    fn rot180() -> Vec<Vector2<i8>> {
        vec![
            Vector2::new(3, 1),
            Vector2::new(2, 1),
            Vector2::new(2, 0),
            Vector2::new(1, 0),
            Vector2::new(0, 0),
        ]
    }

    // x
    // xx
    //  x
    //  x
    fn rot270() -> Vec<Vector2<i8>> {
        vec![
            Vector2::new(0, 3),
            Vector2::new(0, 2),
            Vector2::new(1, 2),
            Vector2::new(1, 1),
            Vector2::new(1, 0),
        ]
    }

    #[test]
    fn should_canonicalize() {
        let canonical_rot0 = ShapeMinimal::new(rot0());
        let canonical_rot90 = ShapeMinimal::new(rot90());
        let canonical_rot180 = ShapeMinimal::new(rot180());
        let canonical_rot270 = ShapeMinimal::new(rot270());

        assert_eq!(canonical_rot0, canonical_rot90);
        assert_eq!(canonical_rot0, canonical_rot180);
        assert_eq!(canonical_rot0, canonical_rot270);

        // non-zero min
        let rot180_minus_one_one = rot180().iter()
            .map(|p| p - Vector2::new(1, 1))
            .collect_vec();

        let rot0_all_minus_one_one = ShapeMinimal::new(rot0().iter()
            .map(|p| p - Vector2::new(1, 1))
            .collect_vec());
        assert_eq!(rot0_all_minus_one_one, canonical_rot0);

        // hashcode
        assert_eq!(hash_shape_minimal(&canonical_rot0), hash_shape_minimal(&canonical_rot90));
        assert_eq!(hash_shape_minimal(&canonical_rot0), hash_shape_minimal(&canonical_rot180));
        assert_eq!(hash_shape_minimal(&canonical_rot0), hash_shape_minimal(&canonical_rot270));
        assert_eq!(hash_shape_minimal(&canonical_rot0), hash_shape_minimal(&rot0_all_minus_one_one));


        // level 2
        let l2_up = ShapeMinimal::new(vec![
            Vector2::new(0, 0),
            Vector2::new(0, 1),
        ]);
        let l2_right = ShapeMinimal::new(vec![
            Vector2::new(0, 0),
            Vector2::new(1, 0),
        ]);
        let l2_down = ShapeMinimal::new(vec![
            Vector2::new(0, 0),
            Vector2::new(0, -1),
        ]);
        let l2_left = ShapeMinimal::new(vec![
            Vector2::new(0, 0),
            Vector2::new(-1, 0),
        ]);
        assert_eq!(hash_shape_minimal(&l2_up), hash_shape_minimal(&l2_right));
        assert_eq!(hash_shape_minimal(&l2_up), hash_shape_minimal(&l2_down));
        assert_eq!(hash_shape_minimal(&l2_up), hash_shape_minimal(&l2_left));
    }

    fn hash_shape_minimal(shape: &ShapeMinimal) -> u64 {
        let mut hasher = DefaultHasher::new();
        shape.hash(&mut hasher);
        hasher.finish()
    }
}