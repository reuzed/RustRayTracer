// Functions to march rays and see what happens

use crate::{
    linalg::vec3::{Vec3, unit_vector},
    ray::Ray,
    raymarching::normal,
};

use super::SdfRef;

const CLOSE_TOLERANCE: f64 = 0.0001;
const MAX_STEPS: usize = 2000;
const MAX_DISTANCE: f64 = 200.0;

pub fn march_simple(ray: Ray, sdf: SdfRef) -> bool {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let unit_dir = unit_vector(ray.direction());
    for _ in 0..MAX_STEPS {
        let d = sdf.distance(pos);
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

pub fn softshadow(ray: Ray, mint: f64, maxt: f64, sdf: SdfRef, penumbra_sharpness: f64) -> f64 {
    // March a ray towards the light, keep track of the most occluded point along the ray going to the light
    // This consists of a point nearby the hit, and nearby something else
    // TODO This can be improved by trying to interpolate the closest position between two raymarch steps,
    // Consider a pair of (point, distance) pairs,
    let mut light: f64 = 1.0;
    let mut t = mint;
    let mut lasth = 0.0;
    for _ in 0..SHADOW_STEPS {
        if t > maxt {
            break;
        }
        let p = ray.at(t);
        let h = sdf.distance(p);

        if h < SHADOW_EPSILON {
            return 0.0;
        }
        t += h;
        // Interpolate soft shadow positions: TODO fix
        // if lasth != 0.0 {
        //     // Code from https://iquilezles.org/articles/rmshadows/
        //     let y = h * h / (2.0 * lasth);
        //     let d = f64::sqrt(h * h - y * y);
        //     light = light.min(penumbra_sharpness * d / (t-y));
        // }
        // Don't interpolate softshadow:
        light = light.min(penumbra_sharpness * h / t);
        // Store distance of previous point
        lasth = h;
    }
    light
}

pub fn march(ray: Ray, sdf: SdfRef) -> MarchResult {
    // Take a ray and sdf, march up to some tolerance or max number of steps
    let mut pos = ray.origin();
    let mut travelled = 0.0;
    let unit_dir = unit_vector(ray.direction());
    let mut steps_taken = 0;
    for i in 0..MAX_STEPS {
        let d = sdf.distance(pos);
        if d < CLOSE_TOLERANCE && travelled > 3.0 * CLOSE_TOLERANCE {
            return MarchResult {
                hit: true,
                steps: i,
                hr: Some(HitResult {
                    travelled: travelled,
                    distance: d,
                    pos: pos,
                    normal: normal(sdf, pos),
                }),
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
