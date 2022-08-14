use shape_core::{Circle, EuclideanDistance, Point, Real, ShapeError};

pub(crate) struct WelzlResolver2D<T> {
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
            // FIxME: The points may overlap, use hashmap?
            points.push(point);
            k += 1;
        }
        Self { points, k, boundary: vec![], circle: Circle { center: Point::default(), radius: T::zero() } }
    }
    pub fn with_smallest_points(self, k: usize) -> Self {
        Self { k, ..self }
    }
    pub fn resolve(mut self) -> Result<Circle<T>, ShapeError> {
        if self.points.len() < self.k {
            return Err(ShapeError::insufficient_points(self.k, self.points.len()));
        }
        self.welzl_recursive()?;
        Ok(self.circle)
    }

    fn welzl_recursive(&mut self) -> Result<(), ShapeError> {
        if self.boundary.len() == self.k {
            self.min_circle_from_boundary()?;
        }
        else {
            let p = self.points.pop().unwrap();
            self.welzl_recursive()?;
            if !self.circle.contains(&p) {
                self.boundary.push(p);
                self.welzl_recursive()?;
                self.boundary.pop();
                self.points.push(p);
            }
        }
        Ok(())
    }

    fn min_circle_from_boundary(&mut self) -> Result<(), ShapeError> {
        match self.boundary.as_mut_slice() {
            [] => Err(ShapeError::insufficient_points(1, 0))?,
            [p1] => {
                self.circle.center = p1.clone();
            }
            [p1, p2] => {
                self.circle = Circle::from_2_points(p1, p2);
            }
            [p1, p2, p3] => {
                self.circle = Circle::from_3_points(p1, p2, p3);
            }
            _ => {
                let mut points = self.boundary.clone();
                let mut center = points.pop().unwrap();
                let radius = T::zero();
                while !points.is_empty() {
                    let p = points.pop().unwrap();
                    let d = center.euclidean_distance(&p);
                    if d > radius {
                        self.circle.radius = d;
                        self.circle.center = p;
                    }
                }
            }
        }
        Ok(())
    }
}
