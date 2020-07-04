use crate::vector;
use vector::Vector;

// P(t) = A + t * b
pub struct Ray {
    pub origin: Vector, // A
    pub direction: Vector, // b
}

// P(t)
impl Ray {
    fn at(self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
