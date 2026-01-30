use rust_ray_tracer::{
    camera::CameraBuilder,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    linalg::vec3::{Point3, Vec3, dot, unit_vector},
    media::{image::save_frame, ppm::ppm_header, renderer::SdfRenderer, screen::Screen},
    random::random_double,
    ray::Ray,
    raymarching::{
        march, repetition, rotate, sd_box, sd_plane, sd_sphere, smooth_union, softshadow,
        translate, union,
    },
    shading::{Color, shade, write_color},
    sphere::Sphere,
};

fn main() {
    // World

    // let sdf = {
    //     let sdf1 = sd_sphere(0.5);
    //     let sdf1 = translate(sdf1, Vec3::new(0.0, 0.0, -1.0));
    //     let sdf2 = sd_box(Vec3::new(0.3, 0.3, 0.3));
    //     let sdf2 = translate(sdf2, Vec3::new(1.0, 0.0, -1.5));
    //     let sdf2 = rotate(sdf2, 5.0, -20.0, 50.0);
    //     let sdf_floor = sd_plane(Vec3::new(0.0, 1.0, 0.0), 0.0);
    //     smooth_union(sdf_floor, union(repetition(sdf1, 5.0), repetition(sdf2, 20.0)), 0.2)
    // };

    let sdf = {
        let sdf_floor = sd_plane(Vec3::new(0.0, 1.0, 0.0), 0.0);
        let sdf_box = sd_box(Vec3::new(0.3, 2.3, 0.3));
        let sdf_box = translate(sdf_box, Vec3::new(0.0, 2.5, 0.0));
        union(sdf_floor, sdf_box)
    };

    // Camera

    let mut camera_builder = CameraBuilder::new();

    camera_builder.target(Vec3::new(0.0, 0.0, -1.0));
    camera_builder.position(Vec3::new(2.0, 2.0, -4.0));

    let screen = Screen::new(512, 16.0 / 9.0);

    const SAMPLES_PER_PIXEL: i32 = 1;

    for i in 0..30 {
        camera_builder.position(Vec3::new(
            5.0 * f64::cos(0.1 * i as f64),
            2.0,
            5.0 * f64::sin(0.1 * i as f64),
        ));
        let camera = camera_builder.build();

        let sdf_renderer = SdfRenderer::new(sdf.clone(), camera, screen.clone());

        let frame = sdf_renderer.monte_carlo_render(SAMPLES_PER_PIXEL);

        save_frame(frame, &format!("tmp/image_{:03}.png", i));
    }
}
