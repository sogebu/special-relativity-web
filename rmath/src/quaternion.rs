use std::ops::Mul;

use crate::{matrix::Matrix, vector::Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion {
    s: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Quaternion {
    pub const fn new(s: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { s, x, y, z }
    }

    pub const fn one() -> Quaternion {
        Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }

    /// Construct a new quaternion as `s[rad]` rotation around 3d-axis
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3, Matrix};
    /// use approx::relative_ne;
    /// // Rotate π/2 around x-axis
    /// let q = Quaternion::from_axis(std::f64::consts::PI / 2.0, Vector3::new(1.0, 0.0, 0.0));
    /// relative_ne!(Matrix::from(q) * Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn from_axis(s: f64, axis: Vector3) -> Quaternion {
        let cos = (s * 0.5).cos();
        let sin = (s * 0.5).sin();
        Quaternion::new(
            cos,
            axis.x as f64 * sin,
            axis.y as f64 * sin,
            axis.z as f64 * sin,
        )
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Self) -> Self::Output {
        let s = self.s * rhs.s - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
        let x = self.s * rhs.x + self.x * rhs.s + self.y * rhs.z - self.z * rhs.y;
        let y = self.s * rhs.y - self.x * rhs.z + self.y * rhs.s + self.z * rhs.x;
        let z = self.s * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.s;
        Quaternion::new(s, x, y, z)
    }
}

impl From<Quaternion> for Matrix {
    /// Convert the quaternion to rotate matrix
    #[rustfmt::skip]
    fn from(q: Quaternion) -> Self {
        let x2 = 2.0 * q.x * q.x;
        let y2 = 2.0 * q.y * q.y;
        let z2 = 2.0 * q.z * q.z;
        let tx = 2.0 * q.s * q.x;
        let ty = 2.0 * q.s * q.y;
        let tz = 2.0 * q.s * q.z;
        let xy = 2.0 * q.x * q.y;
        let yz = 2.0 * q.y * q.z;
        let zx = 2.0 * q.z * q.x;
        Matrix::new(
            (1.0 - y2 - z2) as f32, (xy - tz) as f32, (zx + ty) as f32, 0.0,
            (xy + tz) as f32, (1.0 - z2 - x2) as f32, (yz - tx) as f32, 0.0,
            (zx - ty) as f32, (yz + tx) as f32, (1.0 - x2 - y2) as f32, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}
