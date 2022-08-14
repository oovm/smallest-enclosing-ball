use crate::{welzl::dim2::WelzlResolver2D, EnclosingCircle};
use shape_core::{Circle, Line, Point, PointSet, Real, Shape2D, ShapeError, Square, Triangle};

impl<T> EnclosingCircle<T> for PointSet<T>
where
    T: Real,
{
    fn enclosing(&self) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(self.points.iter().cloned()).resolve()
    }

    fn enclosing_k(&self, k: usize) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(self.points.iter().cloned()).with_smallest_points(k).resolve()
    }
}

impl<T> EnclosingCircle<T> for Line<T>
where
    T: Real,
{
    fn enclosing(&self) -> Result<Circle<T>, ShapeError> {
        Ok(Circle::from_2_points(&self.s, &self.e))
    }

    fn enclosing_k(&self, k: usize) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(vec![self.s, self.e]).with_smallest_points(k).resolve()
    }
}
impl<T> EnclosingCircle<T> for Triangle<T>
where
    T: Real,
{
    fn enclosing(&self) -> Result<Circle<T>, ShapeError> {
        Ok(Circle::from_3_points(&self.a, &self.b, &self.c))
    }

    fn enclosing_k(&self, k: usize) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(vec![self.a, self.b, self.c]).with_smallest_points(k).resolve()
    }
}

impl<T> EnclosingCircle<T> for Square<T>
where
    T: Real,
{
    fn enclosing(&self) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(vec![
            Point { x: self.x.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() },
            Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() },
        ])
        .resolve()
    }

    fn enclosing_k(&self, k: usize) -> Result<Circle<T>, ShapeError> {
        WelzlResolver2D::new(vec![
            Point { x: self.x.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() },
            Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() },
        ])
        .with_smallest_points(k)
        .resolve()
    }
}
