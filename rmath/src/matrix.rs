use std::ops::Mul;

use crate::{angle::Rad, vector::Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Matrix {
    cols: [[f64; 4]; 4],
}

impl Matrix {
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn new(
        r0c0: f64, r0c1: f64, r0c2: f64, r0c3: f64,
        r1c0: f64, r1c1: f64, r1c2: f64, r1c3: f64,
        r2c0: f64, r2c1: f64, r2c2: f64, r2c3: f64,
        r3c0: f64, r3c1: f64, r3c2: f64, r3c3: f64,
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

    pub const fn from_cols(cols: [[f64; 4]; 4]) -> Matrix {
        Matrix { cols }
    }

    /// Get OpenGL matrix format array
    /// * Column major order
    /// * 32 bit floating point number
    ///
    /// ```rust
    /// # use rmath::Matrix;
    /// let m = Matrix::new(
    ///     1.0, 2.0, 3.0, 4.0,
    ///     5.0, 6.0, 7.0, 8.0,
    ///     9.0, 10.0, 11.0, 12.0,
    ///     13.0, 14.0, 15.0, 16.0,
    /// );
    /// assert_eq!(
    ///     m.open_gl(),
    ///     [
    ///         1.0, 5.0, 9.0, 13.0,
    ///         2.0, 6.0, 10.0, 14.0,
    ///         3.0, 7.0, 11.0, 15.0,
    ///         4.0, 8.0, 12.0, 16.0,
    ///     ]
    /// );
    /// ```
    pub const fn open_gl(&self) -> [f32; 16] {
        [
            self.cols[0][0] as f32,
            self.cols[0][1] as f32,
            self.cols[0][2] as f32,
            self.cols[0][3] as f32,
            self.cols[1][0] as f32,
            self.cols[1][1] as f32,
            self.cols[1][2] as f32,
            self.cols[1][3] as f32,
            self.cols[2][0] as f32,
            self.cols[2][1] as f32,
            self.cols[2][2] as f32,
            self.cols[2][3] as f32,
            self.cols[3][0] as f32,
            self.cols[3][1] as f32,
            self.cols[3][2] as f32,
            self.cols[3][3] as f32,
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
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far),
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

    /// Create a Lorentz transform matrix
    #[rustfmt::skip]
    pub fn lorentz(u: Vector3) -> Matrix {
        let x2 = u.x * u.x;
        let y2 = u.y * u.y;
        let z2 = u.z * u.z;
        let r = x2 + y2 + z2;
        if r > 0.0 {
            let g = (1.0 + r).sqrt();
            let r = 1.0 / r;
            let xy = (g - 1.0) * (u.x * u.y) as f64 * r;
            let yz = (g - 1.0) * (u.y * u.z) as f64 * r;
            let zx = (g - 1.0) * (u.z * u.x) as f64 * r;
            Matrix::new(
                (g * x2 + y2 + z2) * r, xy, zx, -u.x,
                xy, (x2 + g * y2 + z2) * r, yz, -u.y,
                zx, yz, (x2 + y2 + g * z2) * r, -u.z,
                -u.x, -u.y, -u.z, g,
            )
        } else {
            Matrix::ident()
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut t = [[0.0; 4]; 4];
        for (c, t) in t.iter_mut().enumerate() {
            for (r, t) in t.iter_mut().enumerate() {
                for i in 0..4 {
                    *t += self.cols[i][r] * rhs.cols[c][i];
                }
            }
        }
        Matrix::from_cols(t)
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        let mut t = [0.0; 3];
        for (i, t) in t.iter_mut().enumerate() {
            *t = self.cols[0][i] * v.x
                + self.cols[1][i] * v.y
                + self.cols[2][i] * v.z
                + self.cols[3][i];
        }
        Vector3::new(t[0], t[1], t[2])
    }
}
