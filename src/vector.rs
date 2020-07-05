use std::ops::Neg;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::ops::AddAssign;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vector;
pub type Point = Vector;

// Operator overloading

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, term: f64) {
        *self = Self {
            x: self.x / term,
            y: self.y / term,
            z: self.z / term,
        };        
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, term: f64) {
        *self = Self {
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
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, term: f64) -> Self {
        Self {
            x: self.x * term,
            y: self.y * term,
            z: self.z * term,
        }
    }       
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, term: f64) -> Self {
        Self {
            x: self.x / term,
            y: self.y / term,
            z: self.z / term,
        }
    }       
}

// Other functions

impl Vector {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,           
        }
    }

    pub fn print_vector(&self) -> () {
        eprintln!("{}, {}, {}", self.x, self.y, self.z);
    }

    pub fn write_color(&self) -> () {
        let x_int = (self.x * 255.999) as i32;
        let y_int = (self.y * 255.999) as i32;
        let z_int = (self.z * 255.999) as i32;
        println!("{} {} {}", x_int, y_int, z_int);
    }
}
