use shape_core::{Circle, Float, Point, Real};
use std::cmp::Ordering;

#[derive(Debug)]
pub enum WelzlError {
    Insufficient,
}
struct WelzlResolver<'i, T> {
    /// All points
    points: &'i [Point<T>],
    /// smallest k points
    k: usize,
    /// boundary points
    boundary: Vec<Point<T>>,
    /// minimum enclosing circle
    circle: Circle<T>,
}

impl<'i, T> WelzlResolver<'i, T>
where
    T: Real,
{
    fn resolve(mut self) -> Result<Circle<T>, WelzlError> {
        if self.points.len() < self.k {
            self.k = self.points.len();
        }
        if self.points.len() <= 1 {
            return Err(WelzlError::Insufficient);
        }
        self.welzl_recursive()?;
        Ok(self.circle)
    }

    fn welzl_recursive(&mut self) -> Result<(), WelzlError> {
        if self.boundary.len() == self.k {
            return Ok(());
        }
        if self.points.is_empty() {
            return Err(WelzlError::Insufficient);
        }
        let point = self.points[self.points.len() - 1];
        if self.boundary.len() < self.k - 1 {
            self.boundary.push(point);
            self.welzl_recursive()?;
            self.boundary.pop();
        }
        let mut updated_circle = self.circle;
        updated_circle.radius = T::zero();
        self.boundary.push(point);
        for i in 0..self.boundary.len() - 1 {
            let p1 = self.boundary[i];
            for j in i + 1..self.boundary.len() {
                let p2 = self.boundary[j];
                let circle = Circle::from_2_points(p1, p2);
                if circle.radius > updated_circle.radius {
                    updated_circle = circle;
                }
            }
        }
        self.boundary.pop();
        self.circle = updated_circle;
        self.welzl_recursive()
    }
}

#[test]
fn main() {
    // 示例数据
    let points = vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 0.0 }, Point { x: 0.5, y: 0.5 }, Point { x: 0.0, y: 1.0 }];
    let circle = welzl(&points, points.len()).unwrap();
    println!("Center: ({}, {}), Radius: {}", circle.center.x, circle.center.y, circle.radius);
}
