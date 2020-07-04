use crate::vector;
use vector::Vector;
use vector::Point;

// P(t) = A + t * b
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point, // A
    pub direction: Vector, // b
}

// P(t)
impl Ray {
    pub fn at(self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
