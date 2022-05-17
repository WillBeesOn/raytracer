use std::fmt::{Display, Formatter};
use std::ops;
use crate::data_structures::Vector;
use crate::{impl_vec_common_methods, impl_vec_ops, Vec3, vec3};

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

#[macro_export]
macro_rules! vec4 {
    ($x: expr, $y: expr, $z:expr, $w:expr) => {
        {
            Vec4 { x: $x, y: $y, z: $z, w: $w }
        }
    };
}

impl Vec4 {
    pub const ZERO: Self = vec4![0.0, 0.0, 0.0, 0.0];

    pub fn from(source: Vec<f64>) -> Self {
        vec4![source[0], source[1], source[2], source[3]]
    }

    pub fn to_vec3(self) -> Vec3 {
        vec3![self.x, self.y, self.z]
    }

    pub fn iter(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }
}

impl Vector for Vec4 {
    fn new(self) -> Self {
        vec4![0.0, 0.0, 0.0, 0.0]
    }

    fn to_vec(self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }

    fn add_v(self, v2: Self) -> Self {
        Vec4 {
            x: self.x + v2.x,
            y: self.y + v2.y,
            z: self.z + v2.z,
            w: self.w + v2.w
        }
    }

    fn add_scalar(self, s: f64) -> Self {
        Vec4 {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
            w: self.w + s
        }
    }

    fn sub_v(self, v2: Self) -> Self {
        Vec4 {
            x: self.x - v2.x,
            y: self.y - v2.y,
            z: self.z - v2.z,
            w: self.w - v2.w
        }
    }

    fn sub_scalar(self, s: f64) -> Self {
        Vec4 {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
            w: self.w - s
        }
    }

    fn mul_v(self, v2: Self) -> Self {
        Vec4 {
            x: self.x * v2.x,
            y: self.y * v2.y,
            z: self.z * v2.z,
            w: self.w * v2.w

        }
    }

    fn mul_scalar(self, s: f64) -> Self {
        Vec4 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s
        }
    }

    fn div_v(self, v2: Self) -> Self {
        Vec4 {
            x: self.x / v2.x,
            y: self.y / v2.y,
            z: self.z / v2.z,
            w: self.w / v2.w
        }
    }

    fn div_scalar(self, s: f64) -> Self {
        Vec4 {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
            w: self.w / s
        }
    }

    fn equals(self, v2: Self) -> bool {
        self.x == v2.x && self.y == v2.y && self.z == v2.z && self.w == v2.w
    }

    fn dot(self, v2: Self) -> f64 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w
    }

    fn abs(self) -> Self {
        vec4![self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs()]
    }

    impl_vec_common_methods!(Vec4);

    fn percent_diff(self, v2: Self) -> f64 {
        let numerator = (self - v2).abs();
        let denominator = (self + v2) / 2.0;
        let diffs = numerator / denominator;
        diffs.dot(vec4![1.0, 1.0, 1.0, 1.0]) / 4.0
    }
}

impl_vec_ops!(Vec4);

impl Display for Vec4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl ops::Index<usize> for Vec4 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("{} is out of bounds for length 3", index)
        }
    }
}

impl ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("{} is out of bounds for length 3", index)
        }
    }
}