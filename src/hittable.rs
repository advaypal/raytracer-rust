use crate::vector;
use crate::ray;
use ray::Ray;
use vector::Vector;
use vector::Point;
use std::option;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
