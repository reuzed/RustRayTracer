// Module for signed disance fields, representing various shapes

use crate::vec3::{Point3, Vec3};

pub trait Sdf: Fn(Vec3) -> f64 + 'static {}
impl <F: Fn(Vec3) -> f64 + 'static> Sdf for F {}

pub fn translate(sdf: impl Sdf, offset: Vec3) -> impl Sdf {
    move |point| sdf(point-offset) 
}

pub fn sd_sphere(radius: f64) -> impl Sdf {
    move |point: Vec3| point.length() - radius
}

pub fn sd_box(size: Vec3) -> impl Sdf {
    move |point: Vec3| {
        let ap = point.abs();
        let q = ap - size;
        q.max(Vec3::zero()).length() + q.max_element().min(0.0)
    }
}