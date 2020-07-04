use crate::vector;
use vector::Vector;
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    fn at(self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
