use shape_core::{Ball, Circle, ShapeError};
use std::error::Error;

pub mod dim2;
pub mod dim3;

pub trait EnclosingCircle<T> {
    /// Returns the smallest enclosing circle of the given points.
    fn enclosing(&self) -> Result<Circle<T>, ShapeError>;
    /// Returns the smallest circle which can enclose k points in given points.
    fn enclosing_k(&self, k: usize) -> Result<Circle<T>, ShapeError>;
}

pub trait EnclosingBall<T> {
    /// Returns the smallest enclosing circle of the given points.
    fn enclosing(&self) -> Result<Ball<T>, ShapeError>;
    /// Returns the smallest circle which can enclose k points in given points.
    fn enclosing_k(&self, k: usize) -> Result<Ball<T>, ShapeError>;
}
