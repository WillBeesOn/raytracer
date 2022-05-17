// Trait for Vec3 and Vec4 to implement
pub trait Vector {
    fn new(self) -> Self;
    fn to_vec(self) -> Vec<f64>;
    fn add_v(self, v2: Self) -> Self;
    fn add_scalar(self, s: f64) -> Self;
    fn sub_v(self, v2: Self) -> Self;
    fn sub_scalar(self, s: f64) -> Self;
    fn mul_v(self, v2: Self) -> Self;
    fn mul_scalar(self, s: f64) -> Self;
    fn div_v(self, v2: Self) -> Self;
    fn div_scalar(self, s: f64) -> Self;
    fn unit(self) -> Self;
    fn length(self) -> f64;
    fn equals(self, v2: Self) -> bool;
    fn dot(self, v2: Self) -> f64;
    fn lerp(self, target: Self, t: f64) -> Self;
    fn abs(self) -> Self;
    fn percent_diff(self, v2: Self) -> f64;
}

// Macro to implement the above methods to overload arithmetic operators (+-*/ and their assign equivalents)
#[macro_export]
macro_rules! impl_vec_ops {
    ($vec_type: ident) => {
        impl ops::Add<$vec_type> for $vec_type {
            type Output = Self;
            fn add(self, v2: $vec_type) -> Self::Output {
                self.add_v(v2)
            }
        }

        impl ops::AddAssign<$vec_type> for $vec_type {
            fn add_assign(&mut self, v2: $vec_type) {
                *self = self.add_v(v2)
            }
        }

        impl ops::Add<f64> for $vec_type {
            type Output = Self;
            fn add(self, s: f64) -> Self::Output {
                self.add_scalar(s)
            }
        }

        impl ops::AddAssign<f64> for $vec_type {
            fn add_assign(&mut self, s: f64) {
                *self = self.add_scalar(s)
            }
        }

        impl ops::Sub<$vec_type> for $vec_type {
            type Output = Self;
            fn sub(self, v2: $vec_type) -> Self::Output {
                self.sub_v(v2)
            }
        }

        impl ops::SubAssign<$vec_type> for $vec_type {
            fn sub_assign(&mut self, v2: $vec_type) {
                *self = self.sub_v(v2)
            }
        }

        impl ops::Sub<f64> for $vec_type {
            type Output = Self;
            fn sub(self, s: f64) -> Self::Output {
                self.sub_scalar(s)
            }
        }

        impl ops::SubAssign<f64> for $vec_type {
            fn sub_assign(&mut self, s: f64) {
                *self = self.sub_scalar(s)
            }
        }

        impl ops::Mul<$vec_type> for $vec_type {
            type Output = Self;
            fn mul(self, v2: $vec_type) -> Self::Output {
                self.mul_v(v2)
            }
        }

        impl ops::MulAssign<$vec_type> for $vec_type {
            fn mul_assign(&mut self, v2: $vec_type) {
                *self = self.mul_v(v2)
            }
        }

        impl ops::Mul<f64> for $vec_type {
            type Output = Self;
            fn mul(self, s: f64) -> Self::Output {
                self.mul_scalar(s)
            }
        }

        impl ops::Mul<$vec_type> for f64 {
            type Output = $vec_type;
            fn mul(self, v: $vec_type) -> Self::Output {
                v.mul_scalar(self)
            }
        }

        impl ops::MulAssign<f64> for $vec_type {
            fn mul_assign(&mut self, s: f64) {
                *self = self.mul_scalar(s)
            }
        }

        impl ops::Div<$vec_type> for $vec_type {
            type Output = Self;
            fn div(self, v2: $vec_type) -> Self::Output {
                self.div_v(v2)
            }
        }

        impl ops::DivAssign<$vec_type> for $vec_type {
            fn div_assign(&mut self, v2: $vec_type) {
                *self = self.div_v(v2)
            }
        }

        impl ops::Div<f64> for $vec_type {
            type Output = Self;
            fn div(self, s: f64) -> Self::Output {
                self.div_scalar(s)
            }
        }

        impl ops::DivAssign<f64> for $vec_type {
            fn div_assign(&mut self, s: f64) {
                *self = self.div_scalar(s)
            }
        }

        impl PartialEq for $vec_type {
            fn eq(&self, v2: &Self) -> bool {
                self.equals(*v2)
            }
        }
    }
}


// Implements the few common Vector trait functions
#[macro_export]
macro_rules! impl_vec_common_methods {
    ($vec_type: ident) => {
        // Returns a unit vector of current vector
        fn unit(self) -> Self {
            self / self.length()
        }

        // Returns length (magnitude) of current vector
        fn length(self) -> f64 {
            self.dot(self).sqrt()
        }

        // Linearly interpolate from current vector (start) to target vector at some time interval t
        fn lerp(self, target: Self, t: f64) -> Self {
            self + target * t
        }
    }
}