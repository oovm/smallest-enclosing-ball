use crate::EnclosingError;
use shape_core::{Circle, Point, Real};

struct WelzlResolver2D<T> {
    /// All points
    points: Vec<Point<T>>,
    /// boundary points
    boundary: Vec<Point<T>>,
    /// minimum enclosing circle
    circle: Circle<T>,
    /// smallest k points
    k: usize,
}

impl<T> WelzlResolver2D<T>
where
    T: Real,
{
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point<T>>,
    {
        let iter = points.into_iter();
        let mut points = Vec::with_capacity(iter.size_hint().0);
        let mut k = 0;
        for point in iter {
            points.push(point);
            k += 1;
        }
        Self { points, k, boundary: vec![], circle: Circle { center: Point::default(), radius: T::zero() } }
    }
    pub fn with_smallest_points(self, k: usize) -> Self {
        Self { k, ..self }
    }
    pub fn resolve(mut self) -> Result<Circle<T>, EnclosingError> {
        if self.points.len() < self.k {
            return Err(EnclosingError::Insufficient { require: self.k, points: self.points.len() });
        }
        self.welzl_recursive()?;
        Ok(self.circle)
    }

    fn welzl_recursive(&mut self) -> Result<(), EnclosingError> {
        if self.boundary.len() == self.k {
            self.circle = self.min_circle_from_points(&self.boundary)?;
            return Ok(());
        }
        if self.points.is_empty() || self.boundary.len() + self.points.len() < self.k {
            return Err(EnclosingError::Insufficient {
                require: self.k - self.boundary.len(),
                points: self.boundary.len() + self.points.len(),
            });
        }
        let point = self.points.pop().unwrap();
        self.welzl_recursive()?;
        if !self.circle.contains(&point) {
            self.boundary.push(point);
            self.welzl_recursive()?;
            self.boundary.pop();
            self.points.push(point);
        }
        Ok(())
    }
    fn min_circle_from_points(&self, points: &[Point<T>]) -> Result<Circle<T>, EnclosingError> {
        match points.len() {
            0 => Err(EnclosingError::Insufficient { require: 1, points: 0 }),
            1 => Ok(Circle::from_1_points(&points[0])),
            2 => Ok(Circle::from_2_points(&points[0], &points[1])),
            3 => Ok(Circle::from_3_points(&points[0], &points[1], &points[2])),
            _ => {
                let mut circle = Circle::from_2_points(&points[0], &points[1]);
                for i in 2..points.len() {
                    if !circle.contains(&points[i]) {
                        circle = Circle::from_2_points(&points[0], &points[i]);
                        for j in 1..i {
                            if !circle.contains(&points[j]) {
                                circle = Circle::from_2_points(&points[i], &points[j]);
                                for k in 0..j {
                                    if !circle.contains(&points[k]) {
                                        circle = Circle::from_3_points(&points[i], &points[j], &points[k]);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(circle)
            }
        }
    }
}

#[test]
fn main() {
    let points = vec![Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 0.0 }, Point { x: 0.5, y: 0.5 }, Point { x: 0.0, y: 1.0 }];
    let circle = WelzlResolver2D::new(points).resolve().unwrap();
    println!("Center: ({}, {}), Radius: {}", circle.center.x, circle.center.y, circle.radius);
}
