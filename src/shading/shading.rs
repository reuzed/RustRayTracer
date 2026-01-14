// Take in information about a point that a ray has intersected with
// Compute how this point should be shaded

use crate::{shading::Color, hittable::HitRecord};

pub fn shade(rec: HitRecord) -> Color {
    if rec.hit {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        Color::new(0.3, 0.3, 0.6)
    }
}
