use shape_core::Circle;

mod dim2;
mod dim3;

pub trait Welzl<T> {
    /// Returns the smallest enclosing circle of the given points.
    fn welzl(&self) -> Circle<T>;
    /// Returns the smallest circle which can enclose k points in given points.
    fn welzl_k(&self, k: usize) -> Circle<T>;
}