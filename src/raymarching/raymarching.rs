// Functions to march rays and see what happens

use std::f64::consts::PI;

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
    pub travelled: f64,
    pub distance: f64,
    pub pos: Vec3,
    pub normal: Vec3,
}

pub struct MarchResult {
    pub hit: bool,
    pub steps: usize,
    pub hr: Option<HitResult>,
}

const SHADOW_STEPS: i32 = 256;
const SHADOW_EPSILON: f64 = 0.001;

pub fn softshadow(ray: Ray, mint: f64, maxt: f64, sdf: impl Sdf, penumbra_sharpness: f64) -> f64 {
    let mut light: f64 = 1.0;
    let mut t = mint;
    for _ in 0..SHADOW_STEPS {
        if t > maxt {
            break;
        }
        let p = ray.at(t);
        let h = sdf(p);
        if h < SHADOW_EPSILON {
            return 0.0;
        }
        t += h;
        light = light.min(penumbra_sharpness * h / t);
    }
    light
}

pub fn march(ray: Ray, sdf: impl Sdf) -> MarchResult {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let mut travelled = 0.0;
    let unit_dir = unit_vector(ray.direction());
    let mut steps_taken = 0;
    for i in 0..MAX_STEPS {
        let d = sdf(pos);
        if d < CLOSE_TOLERANCE && travelled > 3.0 * CLOSE_TOLERANCE {
            return MarchResult {
                hit: true,
                steps: i,
                hr: Some(HitResult{
                    travelled: travelled,
                    distance: d,
                    pos: pos,
                    normal: normal(sdf, pos),
                })
            };
        }
        travelled += d;
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
