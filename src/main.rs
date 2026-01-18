use std::io;

use rust_ray_tracer::{
    camera::{Camera, Renderer}, constants::INFINITY, hittable::{HitRecord, Hittable}, hittable_list::HittableList, linalg::vec3::{self, Point3, Vec3, dot, unit_vector}, media::ppm::ppm_header, random::random_double, ray::Ray, shading::{Color, shade, write_color}, sphere::Sphere
};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // direction = rec.normal + vec3::random_in_unit_sphere();
    if world.hit(&r, 0.0, INFINITY, &mut rec) {
        let direction = rec.normal + vec3::random_in_unit_sphere(); 
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth-1);      // bounce
    }

    // return sky col
    let up = Vec3::new(0.0,1.0,0.0);
    return dot(unit_vector(r.direction()), up).max(0.0) * Vec3::new(1.0,1.0,1.0)
}

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
    const MAX_DEPTH: i32 = 50;
    print!("{}", ppm_header(renderer.image_width, renderer.image_height));

    for j in (0..renderer.image_height).rev() {
        for i in (0..renderer.image_width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (renderer.image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (renderer.image_height - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, MAX_DEPTH)
            }
            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
