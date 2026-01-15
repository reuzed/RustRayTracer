use rust_ray_tracer::{
    camera::{Camera, Renderer}, constants::INFINITY, hittable::{HitRecord, Hittable}, hittable_list::HittableList, linalg::vec3::{Point3, Vec3}, raymarching::{march, rotate, sd_box, sd_sphere, translate, union}, shading::{Color, shade}, sphere::Sphere
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

    let renderer = Renderer::new(512, 16.0 / 9.0, camera);

    // Render

    renderer.render_to_ppm(|ray| {
        let sdf1 = sd_sphere(0.5);
        let sdf1 = translate(sdf1, Vec3::new(0.0, 0.0, -1.0));
        let sdf2 = sd_box(Vec3::new(0.3, 0.3, 0.3));
        let sdf2 = translate(sdf2, Vec3::new(1.0, 0.0, -1.5));
        let sdf2 = rotate(sdf2, 5.0, -20.0, 50.0);
        let sdf = union(sdf1, sdf2);

        let res = march(ray, sdf);

        if res.hit {
            Color::new(1.0,0.3,0.5)
        }
        else {
            Color::new(0.2,0.3,0.5)
        }

    })
}
