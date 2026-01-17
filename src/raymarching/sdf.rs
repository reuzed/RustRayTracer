// Module for signed disance fields, representing various shapes

use std::sync::Arc;

use crate::{
    linalg::{
        vec2::Vec2,
        vec3::{Vec3, dot, unit_vector},
    },
};

pub trait Sdf: Send + Sync {
    fn distance(&self, point: Vec3) -> f64;
}

impl <F> Sdf for F 
where 
    F: Fn(Vec3) -> f64 + Send + Sync,
{
    fn distance(&self, point: Vec3) -> f64 {
        self(point)
    }
}

pub type SdfRef = Arc<dyn Sdf>;

pub fn sdf<F>(f: F) -> SdfRef 
where 
    F: Fn(Vec3) -> f64 + Send + Sync + 'static 
{
    Arc::new(f)
}

// impl<F: Fn(Vec3) -> f64 + Clone + 'static> Sdf for F {}

pub fn sd_sphere(radius: f64) -> SdfRef {
    sdf(move |point| point.length() - radius)
}

pub fn sd_box(size: Vec3) -> SdfRef {
    sdf(move |point| {
        let ap = point.abs();
        let q = ap - size;
        q.max(Vec3::zero()).length() + q.max_element().min(0.0)
    })
}

pub fn sd_torus(t: Vec2) -> SdfRef {
    sdf(move |point: Vec3| {
        let q = Vec2::new(point.xz().length() - t.x(), point.y());
        q.length() - t.y()
    })
}

pub fn sd_plane(n: Vec3, h: f64) -> SdfRef {
    let n = unit_vector(n);
    sdf(move |point: Vec3| dot(point, n) + h)
}
