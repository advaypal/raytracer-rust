use crate::hittable;
use crate::ray;
use crate::vector;
use hittable::HitRecord;
use hittable::Hittable;
use vector::Point;
use ray::Ray;
use std::option;

pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let difference_vector = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = difference_vector.dot(r.direction);
        let c = difference_vector.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let disc_term = discriminant.sqrt();
        let first_root = (-half_b - disc_term) / a;
        let second_root = (-half_b + disc_term) / a;
        let root;
        if first_root < t_max && first_root > t_min {
            root = first_root;
        } else if second_root < t_max && second_root > t_min {
            root = second_root;
        } else {
            return None;
        }

        let record = HitRecord {
            p: r.at(root),
            t: root,
            normal: (r.at(root) - self.center) / self.radius
        };
        Some(record)   
    }
}
