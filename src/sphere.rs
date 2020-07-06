use crate::hittable;
use crate::ray;
use crate::vector;

use hittable::HitRecord;
use hittable::Hittable;
use vector::Point;
use ray::Ray;

pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Hittable for Sphere {
    /* Consider a sphere with center C and radius r.
     *
     * To check if point P is on its surface,
     * Since Vector from Center to point P is (P - C),
     * This condition must be satisfied:
     * length of (P - C) == r
     *
     * Therefore,
     * (P - C) · (P - C) = r ^ 2
     *
     * Point P exists for some t, on a Ray R: R(t) = O + t * d
     * (O + td - C) · (O + td - C) = r ^ 2
     *
     * Expanding gives us a quadratic equation for t
     * (d · d) * t ^ 2 + 2 * d · (A - C) * t + (A - C) · (A - C) - r ^ 2 = 0
     *
     * To solve t,
     * let a = d · d
     * let b = 2 * d · (O - C)
     * let c = (O - C) · (O - C) - r ^ 2
     * solve for: A * t^2 + B * t + C = 0
     */
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Vector C -> O = O - C
        let co = ray.origin - self.center;

        let d = ray.direction;
        let r = self.radius;

        // Quadratic coefficients
        let a = d.dot(d);
        let b = (d * 2.0).dot(co);
        let c = co.dot(co) - r * r;

        let (first_root, second_root) = quadratic_solver(a, b, c)?;

        // Check if in range
        let in_range = |t| t > t_min && t < t_max;

        // Record if in range
        let root = if in_range(first_root) {
            Some(first_root)
        } else if in_range(second_root) {
            Some(second_root)
        } else {
            None
        }?;

        let t = root;

        // Incident ray coordinates
        let p = ray.at(root);

        // Calculate normal
        let outward_normal = (p - self.center) / self.radius;
        let front_face = hittable::update_record_face(ray, outward_normal);
        let normal = hittable::update_record_normal(&front_face, outward_normal);

        // Record the event
        let record = HitRecord {
            p,
            t,
            front_face,
            normal,
        };

        Some(record)   
    }
}

// Solves quadratic equations in the form: ax^2 + bx + c = 0
fn quadratic_solver(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    // b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    // sqrt(b^2 - 4ac) / 2a
    let common = discriminant.sqrt() / (2.0 * a);
    return Some((-b - common, -b + common));
}
