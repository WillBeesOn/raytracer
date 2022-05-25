use std::ops;
use std::fmt::{Display, Formatter};
use crate::data_structures::{Vec4, Vector};
use crate::{impl_vec_common_methods, impl_vec_ops, vec4};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z:expr) => {
        {
            Vec3 { x: $x, y: $y, z: $z }
        }
    };
}

impl Vec3 {
    pub const ZERO: Self = vec3![0.0, 0.0, 0.0];

    pub fn from(self, source: Vec<f64>) -> Self {
        vec3![source[0], source[1], source[2]]
    }

    pub fn cross(self, v2: Self) -> Self {
        Vec3 {
            x: self.y * v2.z - self.z * v2.y,
            y: self.z * v2.x - self.x * v2.z,
            z: self.x * v2.y - self.y * v2.x
        }
    }

    // Returns angle in radians
    pub fn angle(self, v2: Self) -> f64 {
        (self.dot(v2) / (self.length() * v2.length())).acos()
    }

    pub fn to_vec4(self, w: f64) -> Vec4 {
        vec4![self.x, self.y, self.z, w]
    }

    // Given a normal vector of a surface, find the reflected unit vector
    // Assumes the vector being operated on is going inward toward the normal's origin, hence the negation.
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        let incoming = -1.0 * self.unit();
        2.0 * (normal.dot(incoming).max(0.0) * normal) - incoming
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3")
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vec3")
        }
    }
}

impl Vector for Vec3 {
    fn new(self) -> Self {
        vec3![0.0, 0.0, 0.0]
    }

    fn to_vec(self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    fn add_v(self, v2: Self) -> Self {
        Vec3 {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z
        }
    }

    fn add_scalar(self, s: f64) -> Self {
        Vec3 {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s
        }
    }

    fn sub_v(self, v2: Self) -> Self {
        Vec3 {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z
        }
    }

    fn sub_scalar(self, s: f64) -> Self {
        Vec3 {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s
        }
    }

    fn mul_v(self, v2: Self) -> Self {
        Vec3 {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z
        }
    }

    fn mul_scalar(self, s: f64) -> Self {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }

    fn div_v(self, v2: Self) -> Self {
        Vec3 {
            x: self.x / v2.x,
            y: self.y / v2.y,
            z: self.z / v2.z
        }
    }

    fn div_scalar(self, s: f64) -> Self {
        Vec3 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s
        }
    }

    fn equals(self, v2: Self) -> bool {
        self.x == v2.x && self.y == v2.y && self.z == v2.z
    }

    fn dot(self, v2: Self) -> f64 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    impl_vec_common_methods!(Vec3);

    fn abs(self) -> Self {
        vec3![self.x.abs(), self.y.abs(), self.z.abs()]
    }

    fn percent_diff(self, v2: Self) -> f64 {
        let numerator = (self - v2).abs();
        let denominator = (self + v2) / 2.0;
        let diffs = numerator / denominator;
        diffs.dot(vec3![1.0, 1.0, 1.0]) / 3.0
    }
}

impl_vec_ops!(Vec3);