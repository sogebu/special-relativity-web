use std::ops::Mul;

use bytemuck::{Pod, Zeroable};

use crate::{angle::Rad, vector::Vector3};

#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct Matrix {
    cols: [[f32; 4]; 4],
}

impl Matrix {
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn new(
        r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
        r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
        r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
        r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
    ) -> Matrix {
        Matrix {
            cols: [
                [r0c0, r1c0, r2c0, r3c0],
                [r0c1, r1c1, r2c1, r3c1],
                [r0c2, r1c2, r2c2, r3c2],
                [r0c3, r1c3, r2c3, r3c3],
            ],
        }
    }

    pub const fn array(&self) -> [f32; 16] {
        [
            self.cols[0][0],
            self.cols[0][1],
            self.cols[0][2],
            self.cols[0][3],
            self.cols[1][0],
            self.cols[1][1],
            self.cols[1][2],
            self.cols[1][3],
            self.cols[2][0],
            self.cols[2][1],
            self.cols[2][2],
            self.cols[2][3],
            self.cols[3][0],
            self.cols[3][1],
            self.cols[3][2],
            self.cols[3][3],
        ]
    }

    #[rustfmt::skip]
    pub const fn ident() -> Matrix {
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    /// Create a perspective projection matrix.
    ///
    /// This is the equivalent to the [`gluPerspective`] function.
    ///
    /// [`gluPerspective`]: https://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml
    #[rustfmt::skip]
    pub fn perspective<R: Into<Rad>>(fovy: R, aspect: f64, near: f64, far: f64) -> Matrix {
        let f = 1.0 / (fovy.into().0 / 2.0).tan();
        Matrix::new(
            (f / aspect) as f32, 0.0, 0.0, 0.0,
            0.0, f as f32, 0.0, 0.0,
            0.0, 0.0, ((far + near) / (near - far)) as f32, ((2.0 * far * near) / (near - far)) as f32,
            0.0, 0.0, -1.0, 0.0,
        )
    }

    /// Create a translation matrix
    ///
    /// ```rust
    /// # use rmath::{Matrix, Vector3};
    /// # use approx::assert_relative_eq;
    /// let m = Matrix::translation(Vector3::new(1.0, 2.0, 3.0));
    /// assert_relative_eq!(m * Vector3::new(4.0, 5.0, 6.0), Vector3::new(5.0, 7.0, 9.0));
    /// ```
    #[rustfmt::skip]
    pub fn translation(v: Vector3) -> Matrix {
        Matrix::new(
            1.0, 0.0, 0.0, v.x,
            0.0, 1.0, 0.0, v.y,
            0.0, 0.0, 1.0, v.z,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut t = [[0.0; 4]; 4];
        for (c, t) in t.iter_mut().enumerate() {
            for (r, t) in t.iter_mut().enumerate() {
                for i in 0..4 {
                    *t += self.cols[i][r] as f64 * rhs.cols[c][i] as f64;
                }
            }
        }
        Matrix::new(
            t[0][0] as f32,
            t[1][0] as f32,
            t[2][0] as f32,
            t[3][0] as f32,
            t[0][1] as f32,
            t[1][1] as f32,
            t[2][1] as f32,
            t[3][1] as f32,
            t[0][2] as f32,
            t[1][2] as f32,
            t[2][2] as f32,
            t[3][2] as f32,
            t[0][3] as f32,
            t[1][3] as f32,
            t[2][3] as f32,
            t[3][3] as f32,
        )
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        let mut t = [0.0; 3];
        for (i, t) in t.iter_mut().enumerate() {
            *t = self.cols[0][i] as f64 * v.x as f64
                + self.cols[1][i] as f64 * v.y as f64
                + self.cols[2][i] as f64 * v.z as f64
                + self.cols[3][i] as f64;
        }
        Vector3::new(t[0] as f32, t[1] as f32, t[2] as f32)
    }
}
