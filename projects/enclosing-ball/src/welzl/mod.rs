use shape_core::{Ball, Circle};

mod dim2;
mod dim3;

pub trait EnclosingCircle<T> {
    /// Returns the smallest enclosing circle of the given points.
    fn enclosing(&self) -> Circle<T>;
    /// Returns the smallest circle which can enclose k points in given points.
    fn enclosing_k(&self, k: usize) -> Circle<T>;
}

pub trait EnclosingBall<T> {
    /// Returns the smallest enclosing circle of the given points.
    fn enclosing(&self) -> Ball<T>;
    /// Returns the smallest circle which can enclose k points in given points.
    fn enclosing_k(&self, k: usize) -> Ball<T>;
}
