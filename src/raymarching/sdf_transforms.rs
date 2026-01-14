use crate::{linalg::mat3::rot_mat, utils::gradians_to_radians, linalg::vec3::Vec3};

use super::Sdf;


pub fn translate(sdf: impl Sdf, offset: Vec3) -> impl Sdf {
    move |point| sdf(point - offset)
}

// fn transform(sdf: impl Sdf, matrix: [[f64; 3]; 3]) -> impl Sdf {
//     sdf
// }

pub fn rotate(sdf: impl Sdf, theta_x: f64, theta_y: f64, theta_z: f64) -> impl Sdf {
    // Rotate around the z, then y, then x axes by MINUS theta_{} gradians
    let (theta_x, theta_y, theta_z) = (
        gradians_to_radians(theta_x),
        gradians_to_radians(theta_y),
        gradians_to_radians(theta_z),
    );
    move |point| {
        let new_point = rot_mat(-theta_x, -theta_y, -theta_z).mul(point);
        sdf(new_point)
    }
}

pub fn union(sdf1: impl Sdf, sdf2: impl Sdf) -> impl Sdf {
    move |point| sdf1(point).min(sdf2(point))
}