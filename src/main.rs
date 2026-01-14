use rust_ray_tracer::{
    camera::{Camera, Renderer}, constants::INFINITY, hittable::{HitRecord, Hittable}, hittable_list::HittableList, shading::shade, sphere::Sphere, vec3::{Point3, Vec3}
};

fn main() {
    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(
        Vec3::new(0.0, 2.0, -2.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
        2.0,
        16.0 / 9.0,
    );

    let renderer = Renderer::new(512, 16.0 / 9.0, camera);

    // Render

    renderer.render_to_ppm(|ray| {
        let mut rec = HitRecord::new();

        world.hit(&ray, 0.0, INFINITY, &mut rec);
        
        shade(rec)
    }
    )
}
