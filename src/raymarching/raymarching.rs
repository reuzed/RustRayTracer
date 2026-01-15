// Functions to march rays and see what happens

use crate::{
    ray::Ray,
    raymarching::{Sdf, normal},
    linalg::vec3::{Vec3, unit_vector},
};

const CLOSE_TOLERANCE: f64 = 0.0001;
const MAX_STEPS: usize = 2000;
const MAX_DISTANCE: f64 = 200.0;

pub fn march_simple(ray: Ray, sdf: impl Sdf) -> bool {
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

pub struct HitResult {
    pub distance: f64,
    pub pos: Vec3,
    pub normal: Vec3,
}

pub struct MarchResult {
    pub hit: bool,
    pub steps: usize,
    pub hr: Option<HitResult>,
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
                hr: Some(HitResult{
                    distance: d,
                    pos: pos,
                    normal: normal(sdf, pos),
                })
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
        hr: None,
    }
}
