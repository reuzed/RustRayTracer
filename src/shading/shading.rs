// Take in information about a point that a ray has intersected with
// Compute how this point should be shaded

use crate::{
    hittable::HitRecord,
    linalg::vec3::{Vec3, reflect, unit_vector},
    shading::Color,
};

use super::{Light, phong};

pub fn shade(rec: HitRecord) -> Color {
    let light = Light {
        position: Vec3::new(0.5, 3.0, 0.0),
    };
    let to_light = unit_vector(light.position - rec.p);
    let outgoing = unit_vector(reflect(rec.v, rec.normal));
    if rec.hit {
        phong(outgoing, rec.normal, to_light) * Color::new(1.0, 1.0, 1.0)
    } else {
        Color::new(0.3, 0.3, 0.6)
    }
}
