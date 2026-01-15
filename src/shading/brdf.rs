use crate::linalg::vec3::{Vec3, dot};

// Fresnel (shallow angle reflection) : https://en.wikipedia.org/wiki/Fresnel_equations
// https://en.wikipedia.org/wiki/Schlick%27s_approximation

// try these: https://en.wikipedia.org/wiki/List_of_common_shading_algorithms

pub fn lambert(_outgoing: Vec3, normal: Vec3, to_light: Vec3) -> f64 {
    // https://en.wikipedia.org/wiki/Lambertian_reflectance
    dot(to_light, normal).max(0.0)
}

pub fn glossy(outgoing: Vec3, _normal: Vec3, to_light: Vec3) -> f64 {
    dot(to_light, outgoing).max(0.0).powi(4)
}

pub fn phong(outgoing: Vec3, normal: Vec3, to_light: Vec3) -> f64 {
    //https://en.wikipedia.org/wiki/Phong_reflection_model
    0.1 + 0.4 * lambert(outgoing, normal, to_light) + 0.5 * glossy(outgoing, normal, to_light)
}


