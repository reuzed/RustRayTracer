use crate::utils::clamp;
use ::std::io::Write;

use crate::linalg::vec3::Vec3;

// Type Alias
pub type Color = Vec3;

pub fn color_to_string(pixel_color: Color) -> String {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());

    let r = (clamp(r, 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(g, 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(b, 0.0, 0.999) * 256.0) as i32;

    format!("{} {} {}", r, g, b)
}

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    // Write the translated [0,255] value of each color component
    let mut r = pixel_color.x() as f64;
    let mut g = pixel_color.y() as f64;
    let mut b = pixel_color.z() as f64;

    let scale = 1.0 / samples_per_pixel as f64;

    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let r = (clamp(r, 0.0, 0.999) * 256.0) as i32;
    let g = (clamp(g, 0.0, 0.999) * 256.0) as i32;
    let b = (clamp(b, 0.0, 0.999) * 256.0) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing colour");
}
