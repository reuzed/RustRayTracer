use crate::constants::PI;
use crate::linalg::vec3::Vec3;

pub fn lerp(v: Vec3, w: Vec3, t: f64) -> Vec3 {
    t * v + (1.0 - t) * w
}

pub fn remap(t: f64, imin: f64, imax: f64, omin: f64, omax: f64) -> f64 {
    omin + (t - imin) / (imax - imin) * (omax - omin)
}

pub fn gradians_to_radians(gradians: f64) -> f64 {
    gradians * PI / 200.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}

mod tests {
    #[test]

    fn test_clamp() {
        use super::clamp;
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }
}
