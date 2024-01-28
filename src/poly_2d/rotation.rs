use nalgebra::{Matrix2, Rotation2};

pub static ROTATIONS: &[Rotation2<i32>] = &[
    Rotation2::from_matrix_unchecked(Matrix2::new(1, 0, 0, 1)), // 0 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, -1, 1, 0)), // 90 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(-1, 0, 0, -1)), // 180 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, 1, -1, 0)), // 270 deg ccw
];

pub static ROTATIONS8: &[Rotation2<i8>] = &[
    Rotation2::from_matrix_unchecked(Matrix2::new(1, 0, 0, 1)), // 0 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, -1, 1, 0)), // 90 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(-1, 0, 0, -1)), // 180 deg ccw
    Rotation2::from_matrix_unchecked(Matrix2::new(0, 1, -1, 0)), // 270 deg ccw
];