use crate::vec3::{Vec3};

pub fn lerp(v: Vec3, w: Vec3, t: f64) -> Vec3 {
    t * v + (1.0 - t) * w
}