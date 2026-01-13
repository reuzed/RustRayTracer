use std::io;

use rust_ray_tracer::{
    camera::{Camera, Renderer},
    color::{Color, write_color},
    constants::INFINITY,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    utils, vec2,
    vec3::{Point3, Vec3},
};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    return Color::new(0.3, 0.3, 0.6);
}

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        1.0,
        2.0,
        16.0 / 9.0,
    );

    let origin = camera.position.clone();
    let renderer = Renderer::new(512, 16.0 / 9.0, camera);

    // Render

    print!("{}", renderer.ppm_header());

    for dir in renderer.directions_iter() {
        let r = Ray::new(origin, dir);
        let pixel_color = ray_color(&r, &world);
        write_color(&mut io::stdout(), pixel_color);
    }
}
