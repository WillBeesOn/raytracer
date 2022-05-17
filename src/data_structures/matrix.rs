use std::ops;
use std::slice::Iter;
use crate::data_structures::{Vec4, Vector};
use crate::{Vec3, vec4};
use nalgebra::Matrix4 as NalMatrix4;
use crate::Light::PointLight;

// Struct for a matrix with a width of 4. Will typically be used for transformations.
#[derive(Debug)]
pub struct Matrix4 {
    data: Vec<Vec4>
}

// Macro to create a matrix as an Array of Arrays. Converts it to a Matrix of Vectors
#[macro_export]
macro_rules! matrix4 {
    ( $( $x:expr ),* ) => {
        {
            let mut new_matrix = Matrix4::new();
            $(
                new_matrix.push(Vec4::from($x.to_vec()));
            )*
            new_matrix
        }
    };
}

impl Matrix4 {
    pub fn new() -> Self {
        Matrix4 { data: vec![] }
    }

    pub fn from(data: Vec<Vec4>) -> Self {
        Matrix4 { data }
    }

    pub fn push(&mut self, vec: Vec4) {
        self.data.push(vec);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<'_, Vec4> {
        self.data.iter()
    }
    // Transpose, cols become rows, rows become cols. Returns new matrix, does not transpose in place.
    pub fn transpose_square(&self) -> Self {
        if self.data.len() == 4 {
            let mut transposed = Matrix4 { data: vec![Vec4::ZERO; 4]};
            for w in 0..4 {
                for h in 0..4 {
                    transposed.data[w][h] = self.data[h][w];
                }
            }
            transposed
        } else {
            panic!("Can't transpose non-square matrix.")
        }
    }

    // Return multiplicative inverse of this matrix
    pub fn inverse(&self) -> Matrix4 {
        let mut m = Matrix4::new();
        let mut flat = vec![];
        for v in self.data.iter() {
            flat.extend(v.iter());
        }
        let inv = NalMatrix4::from_vec(flat).try_inverse();
        for result in inv.iter() {
            for row in result.data.0.iter() {
                m.push(vec4![row[0], row[1], row[2], row[3]])
            }
        }
        m
    }

}

impl Clone for Matrix4 {
    fn clone(&self) -> Self {
        Matrix4 {
            data: self.data.to_vec()
        }
    }
}

impl ops::Mul<&Matrix4> for &Matrix4 {
    type Output = Matrix4;
    fn mul(self, m2: &Matrix4) -> Self::Output {
        // If they are compatible matrices. Both matrices have a width of 4, but second must have a height (len) of 4.
        if m2.len() == 4 {
            let mut new_rows = vec![];
            let m2_t = m2.transpose_square();

            for my_row in self.data.iter() {
                // New row is the 1st matrix row dotted with each column of 2nd matrix
                let mut new_row = vec![];
                for col in m2_t.data.iter() {
                    new_row.push(my_row.dot(*col));
                }
                new_rows.push(Vec4::from(new_row));
            }
            Matrix4 { data: new_rows }
        } else {
            panic!("Second matrix must be a square 4x4 matrix.")
        }
    }
}

impl ops::Mul<&Matrix4> for Vec4 {
    type Output = Self;

    fn mul(self, matrix: &Matrix4) -> Self::Output {
        let matrix_t = matrix.transpose_square();
        let mut new_vec = vec4![0.0, 0.0, 0.0, 0.0];
        for (i, row) in matrix_t.iter().enumerate() {
            new_vec[i] = self.dot(*row);
        }
        new_vec
    }
}

impl ops::MulAssign<&Matrix4> for Vec4 {
    fn mul_assign(&mut self, matrix: &Matrix4) {
        *self = *self * matrix;
    }
}

impl ops::Mul<&Matrix4> for Vec3{
    type Output = Self;
    fn mul(self, matrix: &Matrix4) -> Self::Output {
        let matrix_t = matrix.transpose_square();
        let v4 = self.to_vec4(1.0);
        let mut new_vec = vec4![0.0, 0.0, 0.0, 0.0];
        for (i, row) in matrix_t.iter().enumerate() {
            new_vec[i] = v4.dot(*row);
        }
        new_vec.to_vec3()
    }
}

impl ops::MulAssign<&Matrix4> for Vec3 {
    fn mul_assign(&mut self, matrix: &Matrix4) {
        *self = *self * matrix;
    }
}

impl ops::AddAssign<Vec4> for Matrix4 {
    fn add_assign(&mut self, v: Vec4) {
        for i in 0..self.data.len() {
            self.data[i] += v;
        }
    }
}

impl ops::MulAssign<Vec4> for Matrix4 {
    fn mul_assign(&mut self, v: Vec4) {
        for i in 0..self.data.len() {
            self.data[i] *= v;
        }
    }
}

impl ops::Index<usize> for Matrix4 {
    type Output = Vec4;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}