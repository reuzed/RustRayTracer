use std::io;

use rust_ray_tracer::{
    camera::{Camera, Renderer},
    constants::INFINITY,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    linalg::vec3::{Point3, Vec3, dot, unit_vector},
    random::random_double,
    ray::Ray,
    raymarching::{
        march, repetition, rotate, sd_box, sd_plane, sd_sphere, smooth_union, softshadow,
        translate, union,
    },
    shading::{Color, shade, write_color},
    sphere::Sphere,
};

use opencv::core::Mat;

fn render_frame() -> Mat {
    let light = Point3::new(1.0, 4.0, 2.0);

    const SAMPLES_PER_PIXEL: i32 = 1;

    let mut frame = Mat::new_rows_cols_with_default();

    print!("{}", renderer.ppm_header());

    for j in (0..renderer.image_height).rev() {
        for i in (0..renderer.image_width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (renderer.image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (renderer.image_height - 1) as f64;

                let ray = camera.get_ray(u, v);

                let res = march(ray, sdf.clone());

                if res.hit {
                    let hr = res.hr.unwrap();

                    let to_light_vec = unit_vector(light - hr.pos);
                    let to_light_ray = Ray::new(hr.pos + 0.0001 * to_light_vec, to_light_vec);

                    let soft_light = softshadow(to_light_ray, 0.1, 300.0, sdf.clone(), 2.0);

                    let normal_light_proportion = dot(hr.normal, to_light_vec);
                    pixel_color += soft_light * normal_light_proportion * Color::new(1.0, 0.3, 0.5)
                } else {
                    // Sky colour
                    pixel_color += Color::new(0.2, 0.3, 0.5)
                }
            }
            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

fn main() {
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
}
