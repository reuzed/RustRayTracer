use crate::linalg::vec3::{Vec3, dot};

#[derive(Debug, Default)]
pub struct Mat3 {
    m: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn new(r0: Vec3, r1: Vec3, r2: Vec3) -> Mat3 {
        Mat3 {
            m: [
                [r0.x(), r0.y(), r0.z()],
                [r1.x(), r1.y(), r1.z()],
                [r2.x(), r2.y(), r2.z()],
            ],
        }
    }

    pub fn zero() -> Mat3 {
        Default::default()
    }

    pub fn id() -> Mat3 {
        Mat3 {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn transpose(&self) -> Mat3 {
        let (r0, r1, r2) = (self.m[0], self.m[1], self.m[2]);
        Mat3 {
            m: [
                [r0[0], r1[0], r2[0]],
                [r0[1], r1[1], r2[1]],
                [r0[2], r1[2], r2[2]],
            ],
        }
    }

    pub fn mat_mul(&self, other: Mat3) -> Mat3 {
        // Return self * other
        // Transpose other to get its column vectors, do vector multiplications, then form into matrix
        let ot = other.transpose();
        Mat3::new(
            self.mul(Vec3::from_arr(ot.m[0])),
            self.mul(Vec3::from_arr(ot.m[1])),
            self.mul(Vec3::from_arr(ot.m[2])),
        )
    }

    pub fn mul(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            dot(Vec3::from_arr(self.m[0]), v),
            dot(Vec3::from_arr(self.m[1]), v),
            dot(Vec3::from_arr(self.m[2]), v),
        )
    }
}

pub fn x_rot_mat(theta: f64) -> Mat3 {
    let c = f64::cos(theta);
    let s = f64::sin(theta);
    Mat3 {
        m: [[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]],
    }
}

pub fn y_rot_mat(theta: f64) -> Mat3 {
    let c = f64::cos(theta);
    let s = f64::sin(theta);
    Mat3 {
        m: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
    }
}

pub fn z_rot_mat(theta: f64) -> Mat3 {
    let c = f64::cos(theta);
    let s = f64::sin(theta);
    Mat3 {
        m: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
    }
}

pub fn rot_mat(theta_x: f64, theta_y: f64, theta_z: f64) -> Mat3 {
    x_rot_mat(theta_x)
        .mat_mul(y_rot_mat(theta_y))
        .mat_mul(z_rot_mat(theta_z))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::vec3::Vec3;

    const EPS: f64 = 1e-12;

    fn assert_vec3_eq(a: Vec3, b: Vec3) {
        assert!((a.x() - b.x()).abs() < EPS);
        assert!((a.y() - b.y()).abs() < EPS);
        assert!((a.z() - b.z()).abs() < EPS);
    }

    #[test]
    fn identity_matrix_vector_mul() {
        let v = Vec3::new(1.2, -3.4, 5.6);
        let id = Mat3::id();

        let result = id.mul(v);
        assert_vec3_eq(result, v);
    }
}
