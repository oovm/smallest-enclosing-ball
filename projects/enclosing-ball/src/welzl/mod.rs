mod dim2;
mod dim3;

pub trait Welzl {
    type Point;
    /// Returns the smallest enclosing circle of the given points.
    fn welzl(points: &[Self::Point]) -> Self;
    /// Returns the smallest circle which can enclose k points in given points.
    fn welzl_k(points: &[Self::Point], k: usize) -> Self;
}