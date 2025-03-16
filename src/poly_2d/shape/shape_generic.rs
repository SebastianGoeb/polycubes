use nalgebra::SVector;
use num::{Integer, Zero};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

pub trait ShapeN<T: Integer + Zero, const D: usize>: Hash + Eq + Send + Sync + Debug
where
    T: Integer + Zero + Clone + Debug + AddAssign + Send + Sync + 'static,
{
    fn new(points: Vec<SVector<T, D>>) -> Self;

    fn points(&self) -> &[SVector<T, D>];
}
