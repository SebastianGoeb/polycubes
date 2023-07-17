use ndarray::{s, Array2, ArrayView2};

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

#[derive(Debug)]
pub struct Shape {
    pub grid: Array2<u8>,
    pub bounds: BoundingBox,
}

impl Shape {
    pub fn crop(&self) -> ArrayView2<u8> {
        let b = self.bounds.clone();
        self.grid.slice(s![b.min_x..=b.max_x, b.min_y..=b.max_y])
    }
}
