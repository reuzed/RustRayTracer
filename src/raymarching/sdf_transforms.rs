use crate::{linalg::mat3::rot_mat, linalg::vec3::Vec3, utils::gradians_to_radians};

use super::{SdfRef, sdf};

pub fn translate(f: SdfRef, offset: Vec3) -> SdfRef {
    sdf(move |point| f.distance(point - offset))
}

// fn transform(sdf: SdfRef, matrix: [[f64; 3]; 3]) -> SdfRef {
//     sdf
// }

pub fn rotate(f: SdfRef, theta_x: f64, theta_y: f64, theta_z: f64) -> SdfRef {
    // Rotate around the z, then y, then x axes by MINUS theta_{} gradians
    let (theta_x, theta_y, theta_z) = (
        gradians_to_radians(theta_x),
        gradians_to_radians(theta_y),
        gradians_to_radians(theta_z),
    );
    sdf(move |point| {
        let new_point = rot_mat(-theta_x, -theta_y, -theta_z).mul(point);
        f.distance(new_point)
    })
}

pub fn union(sdf1: SdfRef, sdf2: SdfRef) -> SdfRef {
    sdf(move |point| sdf1.distance(point).min(sdf2.distance(point)))
}

pub fn smooth_union(sdf1: SdfRef, sdf2: SdfRef, k: f64) -> SdfRef {
    // https://iquilezles.org/articles/smin/
    let k = k * 4.0;
    sdf(move |point| {
        let d1 = sdf1.distance(point);
        let d2 = sdf2.distance(point);
        let h = (k - (d1 - d2).abs()).max(0.0) / k;
        d1.min(d2) - h * h * k * 1.0 / 4.0
    })
}

pub fn repetition(f: SdfRef, s: f64) -> SdfRef {
    sdf(move |point: Vec3| {
        let p = point - s * (point / s).round_xz();
        f.distance(p)
    })
}
