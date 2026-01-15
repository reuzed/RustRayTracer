use rust_ray_tracer::ray;
use rust_ray_tracer::raymarching::march;
use rust_ray_tracer::raymarching::rotate;
use rust_ray_tracer::raymarching::sd_box;
use rust_ray_tracer::raymarching::translate;
use rust_ray_tracer::raymarching::union;
use rust_ray_tracer::shading::write_color;
use rust_ray_tracer::utils;
use rust_ray_tracer::utils::remap;
use rust_ray_tracer::linalg::vec3;

use ray::Ray;
use rust_ray_tracer::linalg::vec3::dot;
use rust_ray_tracer::{shading::Color, raymarching::sd_sphere};
use std::io;
use utils::lerp;
use vec3::Vec3;

// Usage
// cargo run --example shaded_raymarch > shaded_raymarch.ppm

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 512;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in (0..IMAGE_WIDTH).rev() {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let sdf1 = sd_sphere(0.5);
            let sdf1 = translate(sdf1, Vec3::new(0.0, 0.0, -1.0));
            let sdf2 = sd_box(Vec3::new(0.3, 0.3, 0.3));
            let sdf2 = translate(sdf2, Vec3::new(1.0, 0.0, -1.5));
            let sdf2 = rotate(sdf2, 5.0, -20.0, 50.0);
            let sdf = union(sdf1, sdf2);
            let march_result = march(r, sdf);

            let pixel_color: Color = {
                if march_result.hit {
                    let t = dot(march_result.hr.unwrap().normal, Vec3::new(0.0, 0.0, 1.0));
                    lerp(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 0.3, 0.1), t)
                } else {
                    lerp(
                        Color::new(0.7, 0.8, 1.0),
                        Color::new(0.6, 0.6, 0.6),
                        remap(march_result.steps as f64, 0.0, 20.0, 0.0, 1.0),
                    )
                }
            };

            write_color(&mut io::stdout(), pixel_color);
        }
    }
}
