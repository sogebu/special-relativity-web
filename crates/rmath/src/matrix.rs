use std::ops::{Add, Mul};

use crate::{
    angle::Rad,
    vector::{Vector3, Vector4},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub(crate) rows: [[f64; 4]; 4],
}

impl Matrix {
    pub const fn new(row0: [f64; 4], row1: [f64; 4], row2: [f64; 4], row3: [f64; 4]) -> Matrix {
        Matrix {
            rows: [row0, row1, row2, row3],
        }
    }

    pub const fn eta() -> Matrix {
        Matrix::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, -1.0],
        )
    }

    /// Get OpenGL mat4 format array
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

    /// Get OpenGL mat3 format array
    /// * Rotation part
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
    ///     m.open_gl_mat3(),
    ///     [
    ///         1.0, 5.0, 9.0,
    ///         2.0, 6.0, 10.0,
    ///         3.0, 7.0, 11.0,
    ///     ]
    /// );
    /// ```
    pub const fn open_gl_mat3(&self) -> [f32; 9] {
        [
            self.rows[0][0] as f32,
            self.rows[1][0] as f32,
            self.rows[2][0] as f32,
            self.rows[0][1] as f32,
            self.rows[1][1] as f32,
            self.rows[2][1] as f32,
            self.rows[0][2] as f32,
            self.rows[1][2] as f32,
            self.rows[2][2] as f32,
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
    pub const fn translation(v: Vector3) -> Matrix {
        Matrix::new(
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        )
    }

    /// ```rust
    /// # use rmath::{Matrix, Vector3};
    /// let m = Matrix::scale(Vector3::new(2.0, 3.0, 4.0));
    /// let v = Vector3::new(1.5, 2.5, 3.5);
    /// assert_eq!(m * v, Vector3::new(3.0, 7.5, 14.0));
    /// ```
    pub const fn scale(v: Vector3) -> Matrix {
        Matrix::new(
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
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
            let xy = (g - 1.0) * (u.x * u.y) / r;
            let yz = (g - 1.0) * (u.y * u.z) / r;
            let zx = (g - 1.0) * (u.z * u.x) / r;
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

    /// ```rust
    /// # use rmath::Matrix;
    /// let m = Matrix::new(
    ///     [1.0, 2.0, 3.0, 4.0],
    ///     [5.0, 6.0, 7.0, 8.0],
    ///     [9.0, 10.0, 11.0, 12.0],
    ///     [13.0, 14.0, 15.0, 16.0],
    /// );
    /// assert_eq!(m.transposed(), Matrix::new(
    ///     [1.0, 5.0, 9.0, 13.0],
    ///     [2.0, 6.0, 10.0, 14.0],
    ///     [3.0, 7.0, 11.0, 15.0],
    ///     [4.0, 8.0, 12.0, 16.0],
    /// ));
    /// ```
    pub const fn transposed(&self) -> Matrix {
        let [r0, r1, r2, r3] = self.rows;
        Matrix::new(
            [r0[0], r1[0], r2[0], r3[0]],
            [r0[1], r1[1], r2[1], r3[1]],
            [r0[2], r1[2], r2[2], r3[2]],
            [r0[3], r1[3], r2[3], r3[3]],
        )
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

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut mat = self;
        for row in mat.rows.iter_mut() {
            for x in row.iter_mut() {
                *x *= rhs;
            }
        }
        mat
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        let mut mat = self;
        for (lhs, rhs) in mat.rows.iter_mut().zip(rhs.rows.iter()) {
            for (lhs, rhs) in lhs.iter_mut().zip(rhs) {
                *lhs += *rhs;
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: Matrix = Matrix::new(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );

    #[test]
    fn mat_mul() {
        assert_eq!(M * Matrix::ident(), M);
        assert_eq!(Matrix::ident() * M, M);
        assert_eq!(M * Matrix::zero(), Matrix::zero());
        assert_eq!(
            M * M,
            Matrix::new(
                [90., 100., 110., 120.],
                [202., 228., 254., 280.],
                [314., 356., 398., 440.],
                [426., 484., 542., 600.],
            )
        );
    }

    #[test]
    fn mat_vec_mul() {
        assert_eq!(
            M * Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(30.0, 70.0, 110.0, 150.0)
        );
    }

    #[test]
    fn mat_add() {
        let m2 = Matrix::new(
            [1.5, 2.5, 3.5, 4.5],
            [5.5, 6.5, 7.5, 8.5],
            [9.5, 10.5, 11.5, 12.5],
            [13.5, 14.5, 15.5, 16.5],
        );
        assert_eq!(M + Matrix::zero(), M);
        assert_eq!(
            M + m2,
            Matrix::new(
                [2.5, 4.5, 6.5, 8.5],
                [10.5, 12.5, 14.5, 16.5],
                [18.5, 20.5, 22.5, 24.5],
                [26.5, 28.5, 30.5, 32.5],
            )
        );
    }

    #[test]
    fn mul_broadcast() {
        assert_eq!(
            M * -2.0,
            Matrix::new(
                [-2.0, -4.0, -6.0, -8.0],
                [-10.0, -12.0, -14.0, -16.0],
                [-18.0, -20.0, -22.0, -24.0],
                [-26.0, -28.0, -30.0, -32.0],
            )
        );
    }

    #[test]
    fn scale() {
        assert_eq!(
            Matrix::scale(Vector3::new(-0.5, 2.0, 1.5)) * Vector3::new(2.0, 3.0, 4.0),
            Vector3::new(-1.0, 6.0, 6.0)
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
