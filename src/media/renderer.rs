struct SdfRenderer {
    sdf: impl Sdf,
    camera: Camera,
    renderer: Renderer,
    monte_carlo_samples: i32,
}

impl SdfRenderer {
    pub fn new() -> SdfRenderer {
        panic!();
    }

    pub fn render(&self) -> Vec<Vec<Color>> {
        panic!();
    }

    pub fn monte_carlo_render(&self) -> Vec<Vec<Color>> {
        let light = Point3::new(1.0, 4.0, 2.0);

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
                        pixel_color +=
                            soft_light * normal_light_proportion * Color::new(1.0, 0.3, 0.5)
                    } else {
                        // Sky colour
                        pixel_color += Color::new(0.2, 0.3, 0.5)
                    }
                }
                write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
            }
        }
        panic!();
    }

    pub fn output_ppm(&self) {
        print!("{}", renderer.ppm_header());
    }

    fn main() {}
}
