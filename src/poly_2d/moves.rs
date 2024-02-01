use nalgebra::Vector2;

pub static MOVES32: &[Vector2<i32>] = &[
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
];

pub static MOVES8: &[Vector2<i8>] = &[
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(1, 0),
    Vector2::new(-1, 0),
];