use std::io;

use rust_ray_tracer::{
    camera::{Camera, Renderer},
    constants::INFINITY,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    linalg::vec3::{Point3, Vec3},
    random::random_double,
    shading::{Color, shade, write_color},
    sphere::Sphere,
};

fn main() {
    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.5, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.5, 0.3, -2.0), 0.3)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.0, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(
        Vec3::new(2.0, 2.0, -4.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
        2.0,
        16.0 / 9.0,
    );

    let renderer = Renderer::new(512, 16.0 / 9.0, camera.clone());

    // Render

    const SAMPLES_PER_PIXEL: i32 = 100;

    print!("{}", renderer.ppm_header());

    for j in (0..renderer.image_height).rev() {
        for i in (0..renderer.image_width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (renderer.image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (renderer.image_height - 1) as f64;
                let mut rec = HitRecord::new();

                world.hit(&camera.get_ray(u, v), 0.0, INFINITY, &mut rec);

                pixel_color += shade(rec)
            }
            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
