use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, r: f64) -> Sphere {
        Sphere{center: center, radius: r}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = vec3::dot(oc, oc) - self.radius * self.radius;
        let quarter_discriminant = half_b * half_b - a * c;
        if quarter_discriminant < 0.0 {
            return false;
        } 

        let sqrt_d = f64::sqrt(quarter_discriminant);

        // find the nearest root that lies in the acceptable range
        let root1 = (-half_b - sqrt_d) / a;
        let root2 = (-half_b + sqrt_d) / a;
        if  t_min <= root1 && root1 <= t_max {
            rec.t = root1;
        }
        else if t_min <= root2 && root2 <= t_max {
            rec.t = root2;
        }
        else {
            return false;
        }

        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }
}
