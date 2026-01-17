use crate::linalg::vec3::{Point3, Vec3, unit_vector};

use super::SdfRef;

const DELTA: f64 = 0.0001;
pub fn normal(sdf: SdfRef, point: Point3) -> Vec3 {
    let dx = Vec3::new(DELTA, 0.0, 0.0);
    let dy = Vec3::new(0.0, DELTA, 0.0);
    let dz = Vec3::new(0.0, 0.0, DELTA);
    let d = sdf.distance(point);
    unit_vector(Vec3::new(
        d - sdf.distance(point - dx),
        d - sdf.distance(point - dy),
        d - sdf.distance(point - dz),
    ))
}
