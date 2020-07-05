use crate::vector;
use crate::ray;
use ray::Ray;
use vector::Vector;
use vector::Point;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: RayLocale
}

// Whether ray is coming from inside or outside an object
pub enum RayLocale {
    Outside,
    Inside
}

pub fn update_record_face(ray: &Ray, outward_normal: Vector) -> RayLocale {
    match ray.direction.dot(outward_normal) < 0.0 {
        true => RayLocale::Outside,
        false => RayLocale::Inside,
    }
}

pub fn update_record_normal(front_face: &RayLocale, outward_normal: Vector) -> Vector {
    match front_face {
        RayLocale::Outside => outward_normal,
        RayLocale::Inside => -outward_normal,
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
