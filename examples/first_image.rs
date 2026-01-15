// USAGE:
// cargo run --example first_image > image.ppm

use shading::{Color, write_color};
use rust_ray_tracer::shading;

use std::io;

fn main() {
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in (0..IMAGE_WIDTH).rev() {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let pixel_color = Color::new(r, g, b);
            write_color(&mut io::stdout(), pixel_color, 1);
        }
    }
}
