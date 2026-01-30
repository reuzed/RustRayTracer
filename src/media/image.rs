// we can add gamma colour correction here if needed

use image::{Rgb, RgbImage};

use crate::shading::Color;

pub fn save_frame(frame: Vec<Vec<Color>>, filename: &str) {
    let height = frame.len() as u32;
    let width = frame[0].len() as u32;

    let mut img = RgbImage::new(width, height);

    for (y, row) in frame.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            let r = (255.999 * color.x().clamp(0.0, 1.0)) as u8;
            let g = (255.999 * color.y().clamp(0.0, 1.0)) as u8;
            let b = (255.999 * color.z().clamp(0.0, 1.0)) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    img.save(filename).expect("Failed to save image");
}
