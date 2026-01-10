// Functions to march rays and see what happens

use crate::{ray::Ray, sdf::Sdf, vec3::unit_vector};

const CLOSE_TOLERANCE: f64 = 0.00001;
const MAX_STEPS: usize = 20;

pub fn march(ray: Ray, sdf: impl Sdf) -> bool {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let unit_dir = unit_vector(ray.direction());
    for _ in 0..MAX_STEPS {
        let d = sdf(pos);
        if d < CLOSE_TOLERANCE {
            return true;
        }
        pos += unit_dir * d;
    }
    false
}   