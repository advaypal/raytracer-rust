use core::ops::Neg;
use core::ops::Mul;
use core::ops::Add;
use core::ops::Sub;
use core::ops::MulAssign;
use core::ops::DivAssign;
use core::ops::AddAssign;

#[derive(Copy, Clone)]
enum VectorType {
    Point,
    Color,
}

struct Vector {
    vec_type: VectorType,
    x: f64,
    y: f64,
    z: f64,
}

// Operator overloading

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            vec_type: self.vec_type,
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            vec_type: self.vec_type,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, term: f64) {
        *self = Self {
            vec_type: self.vec_type,
            x: self.x / term,
            y: self.y / term,
            z: self.z / term,
        };        
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, term: f64) {
        *self = Self {
            vec_type: self.vec_type,
            x: self.x * term,
            y: self.y * term,
            z: self.z * term,
        };        
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            vec_type: self.vec_type,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            vec_type: self.vec_type,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            vec_type: self.vec_type,
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Vector {
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z + self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}
