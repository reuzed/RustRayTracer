use std::io;

use crate::{
    shading::{Color, write_color},
    ray::Ray,
    linalg::vec3::{Point3, Vec3, cross, orthogonalise, unit_vector},
};

#[derive(Clone)]
pub struct Camera {
    position: Point3,
    target: Point3,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
}

pub struct CameraBuilder {
    position: Point3,
    target: Point3,
    aspect_ratio: f64,
    viewport_width: f64,
    focal_length: f64,
}

pub struct Renderer {
    camera: Camera,
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
}

impl Camera {
    pub fn new(
        pos: Point3,
        target: Point3,
        focal_length: f64,
        viewport_width: f64,
        aspect_ratio: f64,
    ) -> Camera {
        // TODO cache orthonormal basis of camera
        let viewport_height = viewport_width / aspect_ratio;
        Camera {
            position: pos,
            target: target,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
            focal_length: focal_length,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.viewport_width / self.viewport_height
    }

    pub fn horizontal(&self) -> Vec3 {
        // Cross between vector to target and vertical
        // eprint!(
        //     "pos: {}, target: {} \nvert: {} \nto_screen: {} \nhoriz: {}",
        //     self.position,
        //     self.target,
        //     self.vertical(),
        //     self.to_screen_center(),
        //     self.viewport_width * unit_vector(cross(self.to_screen_center(), self.vertical()))
        // );
        self.viewport_width * unit_vector(cross(self.vertical(), self.to_screen_center()))
    }

    pub fn vertical(&self) -> Vec3 {
        // Positive y-axis vector minus its projection onto vector pointing at target, normalised
        // Special case for camera pointing straight up or down...
        let up = Vec3::new(0.0, 1.0, 0.0);
        let to_screen = self.to_screen_center();
        let vertical_vec = orthogonalise(up, vec![to_screen]);
        self.viewport_height * unit_vector(vertical_vec)
    }

    pub fn to_screen_center(&self) -> Vec3 {
        self.focal_length * unit_vector(self.target - self.position)
    }

    pub fn lower_left_corner(&self) -> Vec3 {
        self.position + self.to_screen_center() - self.horizontal() / 2.0 - self.vertical() / 2.0
    }

    pub fn get_ray(&self, u:f64, v: f64) -> Ray {
        // u, v \in [0,1] describe where on viewport ray will be cast
        let direction = self.lower_left_corner() + u * self.horizontal() + v * self.vertical() - self.position;
        Ray::new(self.position, direction)
    }
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            position: Vec3::new(1.0, 1.0, 1.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
            viewport_width: 2.0,
            focal_length: 1.0,
        }
    }

    pub fn position(mut self, pos: Vec3) {
        self.position = pos
    }

    pub fn target(mut self, target: Vec3) {
        self.target = target
    }

    pub fn build(&self) -> Camera {
        Camera::new(
            self.position,
            self.target,
            self.focal_length,
            self.viewport_width,
            self.aspect_ratio,
        )
    }
}

impl Renderer {
    pub fn new(image_width: u32, aspect_ratio: f64, camera: Camera) -> Renderer {
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        Renderer {
            camera,
            aspect_ratio,
            image_width,
            image_height,
        }
    }

    pub fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.image_width, self.image_height)
    }

    pub fn rays_iter(&self) -> impl Iterator<Item = Ray> {
        (0..self.image_height).rev().flat_map(move |j| {
            (0..self.image_width).rev().map(move |i| {
                let u = i as f64 / (self.image_width - 1) as f64;
                let v = j as f64 / (self.image_height - 1) as f64;
                self.camera.get_ray(u, v)
            })
        })
    }

    pub fn render_to_ppm(&self, ray_to_col: impl Fn(Ray) -> Color) {
        print!("{}", self.ppm_header());

        for ray in self.rays_iter() {
            let pixel_color: Color = ray_to_col(ray);
            write_color(&mut io::stdout(), pixel_color, 1);
        }
    }
}
