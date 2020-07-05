use crate::hittable;
use hittable::Hittable;
use hittable::HitRecord;
use crate::ray;
use ray::Ray;

pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: Vec::new() }
    }
    
    pub fn add(&mut self, object: &'a dyn Hittable) -> () {
        self.objects.push(object);
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }       
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // f64 doesn't implement std::cmp::Ord, so can't use
        // min_by_key
        self.objects.iter()
            .filter_map(|&object| object.hit(r, t_min, t_max))  
            .fold(None, |acc_opt, hit_record| {
                match (acc_opt, hit_record) {
                    (None, hit_record) => Some(hit_record),
                    (Some(acc), hit_record) => {
                        if hit_record.t < acc.t {
                            Some(hit_record)
                        } else {
                            Some(acc)
                        }
                    }
                }
            })
    }
}
