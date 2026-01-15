// Module for signed disance fields, representing various shapes

use crate::{
    linalg::{mat3::rot_mat, vec2::Vec2, vec3::{Point3, Vec3, dot, unit_vector}},
    utils::gradians_to_radians,
};

pub trait Sdf: Fn(Vec3) -> f64 + Clone + 'static {}
impl<F: Fn(Vec3) -> f64 + Clone + 'static> Sdf for F {}

pub fn sd_sphere(radius: f64) -> impl Sdf {
    move |point| point.length() - radius
}

pub fn sd_box(size: Vec3) -> impl Sdf {
    move |point| {
        let ap = point.abs();
        let q = ap - size;
        q.max(Vec3::zero()).length() + q.max_element().min(0.0)
    }
}

pub fn sd_torus(t: Vec2) -> impl Sdf {
    move |point: Vec3| {
        let q = Vec2::new(point.xz().length() - t.x(), point.y());
        q.length() - t.y()
    }
}

pub fn sd_plane(n: Vec3, h: f64) -> impl Sdf {
    let n = unit_vector(n);
    move |point: Vec3| {
        dot(point, n) + h
    }
}

const DELTA: f64 = 0.0001;
pub fn normal(sdf: impl Sdf, point: Point3) -> Vec3 {
    let dx = Vec3::new(DELTA, 0.0, 0.0);
    let dy = Vec3::new(0.0, DELTA, 0.0);
    let dz = Vec3::new(0.0, 0.0, DELTA);
    let d = sdf(point);
    unit_vector(Vec3::new(
        d - sdf(point - dx),
        d - sdf(point - dy),
        d - sdf(point - dz),
    ))
}
