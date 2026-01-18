use std::io;

use crate::{camera::{Camera, Renderer}, linalg::vec3::{Point3, dot, unit_vector}, random::random_double, ray::Ray, raymarching::{Sdf, SdfRef, march, softshadow}, shading::{Color, color_to_string, write_color}};

pub struct SdfRenderer {
    sdf: SdfRef,
    camera: Camera,
    renderer: Renderer,
}

impl SdfRenderer {
    pub fn new(sdf: SdfRef, camera: Camera, renderer: Renderer) -> SdfRenderer {
        SdfRenderer { sdf: sdf, camera: camera, renderer: renderer }
    }

    // pub fn render(&self) -> Vec<Vec<Color>> {
    //     panic!();
    // }

    pub fn monte_carlo_render(&self, samples_per_pixel: i32) -> Vec<Vec<Color>> {
        let mut frame = Vec::new();

        let light = Point3::new(1.0, 4.0, 2.0);

        for j in (0..self.renderer.image_height).rev() {
            let mut row: Vec<Color> = Vec::new();
            for i in (0..self.renderer.image_width).rev() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random_double()) / (self.renderer.image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (self.renderer.image_height - 1) as f64;

                    let ray = self.camera.get_ray(u, v);

                    let res = march(ray, self.sdf.clone());

                    if res.hit {
                        let hr = res.hr.unwrap();

                        let to_light_vec = unit_vector(light - hr.pos);
                        let to_light_ray = Ray::new(hr.pos + 0.0001 * to_light_vec, to_light_vec);

                        let soft_light = softshadow(to_light_ray, 0.1, 300.0, self.sdf.clone(), 2.0);

                        let normal_light_proportion = dot(hr.normal, to_light_vec);
                        pixel_color +=
                            soft_light * normal_light_proportion * Color::new(1.0, 0.3, 0.5)
                    } else {
                        // Sky colour
                        pixel_color += Color::new(0.2, 0.3, 0.5)
                    }
                }
                row.push(pixel_color / (samples_per_pixel as f64));
            }
            frame.push(row);
        }
        frame
    }

    pub fn output_ppm(&self, frame: Vec<Vec<Color>>) {
        print!("{}", self.renderer.ppm_header());

        for row in frame {
            for pixel_color in row {
                println!("{}", color_to_string(pixel_color))
            }
        }
    }
}
