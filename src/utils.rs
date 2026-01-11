use crate::vec3::Vec3;

pub fn lerp(v: Vec3, w: Vec3, t: f64) -> Vec3 {
    t * v + (1.0 - t) * w
}

pub fn remap(t: f64, imin: f64, imax: f64, omin: f64, omax: f64) -> f64 {
    omin + (t - imin) / (imax - imin) * (omax - omin)
}
