use rust_ray_tracer::{
    camera::{Camera, Renderer}, constants::INFINITY, hittable::{HitRecord, Hittable}, hittable_list::HittableList, linalg::vec3::{Point3, Vec3, dot, unit_vector}, ray::Ray, raymarching::{march, rotate, sd_box, sd_plane, sd_sphere, smooth_union, translate, union}, shading::{Color, shade}, sphere::Sphere
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

    let sdf = {
        let sdf1 = sd_sphere(0.5);
        let sdf1 = translate(sdf1, Vec3::new(0.0, 0.0, -1.0));
        let sdf2 = sd_box(Vec3::new(0.3, 0.3, 0.3));
        let sdf2 = translate(sdf2, Vec3::new(1.0, 0.0, -1.5));
        let sdf2 = rotate(sdf2, 5.0, -20.0, 50.0);
        let sdf_floor = sd_plane(Vec3::new(0.0, 1.0, 0.0), 0.0);
        smooth_union(sdf_floor, union(sdf1, sdf2), 0.2)
    };

    let light = Point3::new(1.0, 4.0, 2.0);

    renderer.render_to_ppm(|ray| {
        let res = march(ray, sdf.clone());

        if res.hit {
            let hr = res.hr.unwrap();

            let to_light_vec = unit_vector(light - hr.pos);
            let to_light = Ray::new(hr.pos + 0.1 * to_light_vec, to_light_vec);
            let shadow_res = march(to_light, sdf.clone());
            if shadow_res.hit {
                Color::new(0.1,0.1,0.1)
            }
            else {
                Color::new(1.0,0.3,0.5) * dot(unit_vector(light - hr.pos), hr.normal)
            }
        }
        else {
            Color::new(0.2,0.3,0.5)
        }

    })
}
