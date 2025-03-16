use std::cmp;

use nalgebra::{Rotation2, Vector2};

#[derive(PartialEq, Eq, Debug)]
pub struct BoundingBoxTwoPoints {
    pub p0: Vector2<i32>,
    pub p1: Vector2<i32>,
}

impl BoundingBoxTwoPoints {
    pub fn from(points: &[Vector2<i32>]) -> BoundingBoxTwoPoints {
        BoundingBoxTwoPoints {
            p0: Vector2::new(
                points.iter().map(|p| p.x).min().unwrap(),
                points.iter().map(|p| p.y).min().unwrap(),
            ),
            p1: Vector2::new(
                points.iter().map(|p| p.x).max().unwrap(),
                points.iter().map(|p| p.y).max().unwrap(),
            ),
        }
    }

    pub fn min(&self) -> Vector2<i32> {
        Vector2::new(
            cmp::min(self.p0.x, self.p1.x),
            cmp::min(self.p0.y, self.p1.y),
        )
    }

    pub fn max(&self) -> Vector2<i32> {
        Vector2::new(
            cmp::max(self.p0.x, self.p1.x),
            cmp::max(self.p0.y, self.p1.y),
        )
    }
}

impl std::ops::Mul<&BoundingBoxTwoPoints> for &Rotation2<i32> {
    type Output = BoundingBoxTwoPoints;

    fn mul(self, rhs: &BoundingBoxTwoPoints) -> Self::Output {
        BoundingBoxTwoPoints {
            p0: self * rhs.p0,
            p1: self * rhs.p1,
        }
    }
}

impl std::ops::Add<Vector2<i32>> for BoundingBoxTwoPoints {
    type Output = BoundingBoxTwoPoints;

    fn add(self, rhs: Vector2<i32>) -> BoundingBoxTwoPoints {
        BoundingBoxTwoPoints {
            p0: self.p0 + rhs,
            p1: self.p1 + rhs,
        }
    }
}

impl std::ops::Sub<Vector2<i32>> for BoundingBoxTwoPoints {
    type Output = BoundingBoxTwoPoints;

    fn sub(self, rhs: Vector2<i32>) -> BoundingBoxTwoPoints {
        BoundingBoxTwoPoints {
            p0: self.p0 - rhs,
            p1: self.p1 - rhs,
        }
    }
}
