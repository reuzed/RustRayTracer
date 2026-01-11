// Functions to march rays and see what happens

use crate::{
    ray::Ray,
    sdf::{Sdf, normal},
    vec3::{Vec3, unit_vector},
};

const CLOSE_TOLERANCE: f64 = 0.00001;
const MAX_STEPS: usize = 200;
const MAX_DISTANCE: f64 = 100.0;

pub fn march_simple(ray: Ray, sdf: impl Sdf) -> bool {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let unit_dir = unit_vector(ray.direction());
    for i in 0..MAX_STEPS {
        let d = sdf(pos);
        if d < CLOSE_TOLERANCE {
            return true;
        }
        pos += unit_dir * d;
    }
    false
}

pub struct MarchResult {
    pub hit: bool,
    pub steps: usize,
    pub normal: Option<Vec3>,
}
pub fn march(ray: Ray, sdf: impl Sdf) -> MarchResult {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let unit_dir = unit_vector(ray.direction());
    let mut steps_taken = 0;
    for i in 0..MAX_STEPS {
        let d = sdf(pos);
        if d < CLOSE_TOLERANCE {
            return MarchResult {
                hit: true,
                steps: i,
                normal: Some(normal(sdf, pos)),
            };
        }
        pos += unit_dir * d;

        steps_taken += 1;
        if pos.length() > MAX_DISTANCE {
            break;
        }
    }
    MarchResult {
        hit: false,
        steps: steps_taken,
        normal: None,
    }
}
