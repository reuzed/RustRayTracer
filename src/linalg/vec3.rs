use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::linalg::vec2;

use vec2::Vec2;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn from_arr(r: [f64; 3]) -> Vec3 {
        Vec3 { e: r }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }

    pub fn yz(&self) -> Vec2 {
        Vec2::new(self.y(), self.z())
    }

    pub fn xz(&self) -> Vec2 {
        Vec2::new(self.x(), self.z())
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn abs(&self) -> Vec3 {
        Vec3::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn max_element(&self) -> f64 {
        self.x().max(self.y().max(self.z()))
    }

    pub fn max(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
        )
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

// Type alias
pub type Point3 = Vec3;

// Output Formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// Constant multiplication

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -1.0 * self
    }
}

// f64 * Vec3

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(t * self.x(), t * self.y(), t * self.z())
    }
}

// Vec3 * f64

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

// Vec3 / f64

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

// Binary Operators

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// Special Vector Method
// dot
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

// cross
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

// unit vector
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// project one vector onto another
pub fn project(v: Vec3, n: Vec3) -> Vec3 {
    // return the component of v in the direction of n
    let n = unit_vector(n);
    let vdn = dot(n, v);
    vdn * n
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * project(v, n)
}

pub fn orthogonalise(v: Vec3, b: Vec<Vec3>) -> Vec3 {
    // remove the components of v in the directions in b, giving an orthogonal version
    let mut ov = v;
    for bv in &b {
        ov = ov - project(ov, *bv);
    }
    ov
}
