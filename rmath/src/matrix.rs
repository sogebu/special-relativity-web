use std::ops::Mul;

use crate::{
    angle::Rad,
    vector::{Vector3, Vector4},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    rows: [[f64; 4]; 4],
}

impl Matrix {
    pub const fn new(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Matrix {
        Matrix {
            rows: [row0, row1, row2, row3],
        }
    }

    /// Get OpenGL matrix format array
    /// * Column major order
    /// * 32 bit floating point number
    ///
    /// ```rust
    /// # use rmath::Matrix;
    /// let m = Matrix::new(
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0],
    ///     [13.0, 14.0, 15.0, 16.0],
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
            self.rows[0][0] as f32,
            self.rows[1][0] as f32,
            self.rows[2][0] as f32,
            self.rows[3][0] as f32,
            self.rows[0][1] as f32,
            self.rows[1][1] as f32,
            self.rows[2][1] as f32,
            self.rows[3][1] as f32,
            self.rows[0][2] as f32,
            self.rows[1][2] as f32,
            self.rows[2][2] as f32,
            self.rows[3][2] as f32,
            self.rows[0][3] as f32,
            self.rows[1][3] as f32,
            self.rows[2][3] as f32,
            self.rows[3][3] as f32,
        ]
    }

    pub const fn ident() -> Matrix {
        Matrix::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    pub const fn zero() -> Matrix {
        Matrix::new([0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4])
    }

    /// Create a perspective projection matrix.
    ///
    /// This is the equivalent to the [`gluPerspective`] function.
    ///
    /// [`gluPerspective`]: https://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml
    pub fn perspective<R: Into<Rad>>(fovy: R, aspect: f64, near: f64, far: f64) -> Matrix {
        let f = 1.0 / (fovy.into().0 / 2.0).tan();
        Matrix::new(
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                (2.0 * far * near) / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
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
    pub fn translation(v: Vector3) -> Matrix {
        Matrix::new(
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// Create a Lorentz transform matrix
    ///
    /// The matrix ``Matrix::lorentz(u)`` makes velocity vector ``u`` to zero-vector.
    ///
    /// ```rust
    /// # use rmath::{Matrix, Vector3, Vector4};
    /// # use approx::assert_relative_eq;
    /// let u = Vector3::new(0.1, 0.2, 0.3);
    /// let m = Matrix::lorentz(u);
    /// assert_relative_eq!(
    ///     m * Vector4::from_velocity(u),
    ///     Vector4::from_velocity(Vector3::zero()),
    /// );
    /// ```
    pub fn lorentz(u: Vector3) -> Matrix {
        let x2 = u.x * u.x;
        let y2 = u.y * u.y;
        let z2 = u.z * u.z;
        let r = x2 + y2 + z2;
        if r > 0.0 {
            let g = (1.0 + r).sqrt();
            let xy = (g - 1.0) * (u.x * u.y) as f64 / r;
            let yz = (g - 1.0) * (u.y * u.z) as f64 / r;
            let zx = (g - 1.0) * (u.z * u.x) as f64 / r;
            Matrix::new(
                [(g * x2 + y2 + z2) / r, xy, zx, -u.x],
                [xy, (x2 + g * y2 + z2) / r, yz, -u.y],
                [zx, yz, (x2 + y2 + g * z2) / r, -u.z],
                [-u.x, -u.y, -u.z, g],
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
        for (r, t) in t.iter_mut().enumerate() {
            for (c, t) in t.iter_mut().enumerate() {
                for i in 0..4 {
                    *t += self.rows[r][i] * rhs.rows[i][c];
                }
            }
        }
        Matrix::new(t[0], t[1], t[2], t[3])
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        let mut t = [0.0; 3];
        for (t, row) in t.iter_mut().zip(&self.rows) {
            *t = row[0] * v.x + row[1] * v.y + row[2] * v.z + row[3];
        }
        Vector3::new(t[0], t[1], t[2])
    }
}

impl Mul<Vector4> for Matrix {
    type Output = Vector4;
    fn mul(self, v: Vector4) -> Self::Output {
        let mut t = [0.0; 4];
        for (t, row) in t.iter_mut().zip(&self.rows) {
            *t = row[0] * v.x + row[1] * v.y + row[2] * v.z + row[3] * v.t;
        }
        Vector4::new(t[0], t[1], t[2], t[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat_mul() {
        let m = Matrix::new(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );
        assert_eq!(m * Matrix::ident(), m);
        assert_eq!(Matrix::ident() * m, m);
        assert_eq!(m * Matrix::zero(), Matrix::zero());
        assert_eq!(
            m * m,
            Matrix::new(
                [90., 100., 110., 120.],
                [202., 228., 254., 280.],
                [314., 356., 398., 440.],
                [426., 484., 542., 600.],
            )
        );
    }

    #[test]
    fn translation() {
        assert_eq!(
            Matrix::translation(Vector3::new(1.0, 2.0, 3.0)) * Vector3::new(-4.0, 5.0, 7.0),
            Vector3::new(-3.0, 7.0, 10.0)
        );
    }
}
