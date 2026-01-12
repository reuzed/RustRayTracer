use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut diamond_mint = t_max;
        let mut hit_something = false;
        for obj in &self.objects {
            if obj.hit(ray, t_min, diamond_mint, &mut temp_rec) {
                hit_something = true;
                diamond_mint = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_something
    }
}
