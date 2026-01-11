// Module for signed disance fields, representing various shapes

use crate::{
    vec2::Vec2,
    vec3::{Point3, Vec3, unit_vector},
};

pub trait Sdf: Fn(Vec3) -> f64 + 'static {}
impl<F: Fn(Vec3) -> f64 + 'static> Sdf for F {}

pub fn translate(sdf: impl Sdf, offset: Vec3) -> impl Sdf {
    move |point| sdf(point - offset)
}

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

pub fn union(sdf1: impl Sdf, sdf2: impl Sdf) -> impl Sdf {
    move |point| sdf1(point).min(sdf2(point))
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
