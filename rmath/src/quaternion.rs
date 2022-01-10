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
    /// let q = Quaternion::from_axis(std::f64::consts::PI / 2.0, Vector3::new(2.0, 0.0, 0.0));
    /// relative_ne!(Matrix::from(q) * Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn from_axis(s: f64, axis: Vector3) -> Quaternion {
        let cos = (s * 0.5).cos();
        let sin = (s * 0.5).sin();
        let length = axis.magnitude() as f64;
        Quaternion::new(
            cos,
            axis.x as f64 * sin / length,
            axis.y as f64 * sin / length,
            axis.z as f64 * sin / length,
        )
    }

    /// Get right-direction vector
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::relative_eq;
    /// let q = Quaternion::one();
    /// relative_eq!(q.right(), Vector3::new(1.0, 0.0, 0.0));
    /// ```
    pub fn right(&self) -> Vector3 {
        let y2 = 2.0 * self.y * self.y;
        let z2 = 2.0 * self.z * self.z;
        let sy = 2.0 * self.s * self.y;
        let sz = 2.0 * self.s * self.z;
        let xy = 2.0 * self.x * self.y;
        let zx = 2.0 * self.z * self.x;
        Vector3::new((1.0 - y2 - z2) as f32, (xy - sz) as f32, (zx + sy) as f32)
    }

    /// Get up-direction vector
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::relative_eq;
    /// let q = Quaternion::one();
    /// relative_eq!(q.up(), Vector3::new(0.0, 1.0, 0.0));
    /// ```
    pub fn up(&self) -> Vector3 {
        let x2 = 2.0 * self.x * self.x;
        let z2 = 2.0 * self.z * self.z;
        let sx = 2.0 * self.s * self.x;
        let sz = 2.0 * self.s * self.z;
        let xy = 2.0 * self.x * self.y;
        let yz = 2.0 * self.y * self.z;
        Vector3::new((xy + sz) as f32, (1.0 - z2 - x2) as f32, (yz - sx) as f32)
    }

    /// Get front-direction vector
    ///
    /// Note: In OpenGL, "front" is often the -z direction.
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::relative_eq;
    /// let q = Quaternion::one();
    /// relative_eq!(q.front(), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn front(&self) -> Vector3 {
        let x2 = 2.0 * self.x * self.x;
        let y2 = 2.0 * self.y * self.y;
        let sx = 2.0 * self.s * self.x;
        let sy = 2.0 * self.s * self.y;
        let yz = 2.0 * self.y * self.z;
        let zx = 2.0 * self.z * self.x;
        Vector3::new((zx - sy) as f32, (yz + sx) as f32, (1.0 - x2 - y2) as f32)
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
        let sx = 2.0 * q.s * q.x;
        let sy = 2.0 * q.s * q.y;
        let sz = 2.0 * q.s * q.z;
        let xy = 2.0 * q.x * q.y;
        let yz = 2.0 * q.y * q.z;
        let zx = 2.0 * q.z * q.x;
        Matrix::new(
            (1.0 - y2 - z2) as f32, (xy - sz) as f32, (zx + sy) as f32, 0.0,
            (xy + sz) as f32, (1.0 - z2 - x2) as f32, (yz - sx) as f32, 0.0,
            (zx - sy) as f32, (yz + sx) as f32, (1.0 - x2 - y2) as f32, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}
