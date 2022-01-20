use std::ops::{Mul, MulAssign};

use crate::{angle::Rad, matrix::Matrix, vector::Vector3};

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

    /// Construct a new quaternion that rotate around 3d-axis
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3, Matrix, Deg};
    /// use approx::assert_relative_eq;
    /// // Rotate π/2 around x-axis
    /// let q = Quaternion::from_axis(Deg(90.0), Vector3::new(2.0, 0.0, 0.0));
    /// assert_relative_eq!(Matrix::from(q) * Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn from_axis<R: Into<Rad>>(s: R, axis: Vector3) -> Quaternion {
        let (sin, cos) = (s.into().0 * 0.5).sin_cos();
        let length = axis.magnitude();
        Quaternion::new(
            cos,
            axis.x * sin / length,
            axis.y * sin / length,
            axis.z * sin / length,
        )
    }

    /// Get right-direction vector
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::assert_relative_eq;
    /// let q = Quaternion::one();
    /// assert_relative_eq!(q.right(), Vector3::new(1.0, 0.0, 0.0));
    /// ```
    pub fn right(&self) -> Vector3 {
        let y2 = 2.0 * self.y * self.y;
        let z2 = 2.0 * self.z * self.z;
        let sy = 2.0 * self.s * self.y;
        let sz = 2.0 * self.s * self.z;
        let xy = 2.0 * self.x * self.y;
        let zx = 2.0 * self.z * self.x;
        Vector3::new(1.0 - y2 - z2, xy - sz, zx + sy)
    }

    /// Get up-direction vector
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::assert_relative_eq;
    /// let q = Quaternion::one();
    /// assert_relative_eq!(q.up(), Vector3::new(0.0, 1.0, 0.0));
    /// ```
    pub fn up(&self) -> Vector3 {
        let x2 = 2.0 * self.x * self.x;
        let z2 = 2.0 * self.z * self.z;
        let sx = 2.0 * self.s * self.x;
        let sz = 2.0 * self.s * self.z;
        let xy = 2.0 * self.x * self.y;
        let yz = 2.0 * self.y * self.z;
        Vector3::new(xy + sz, 1.0 - z2 - x2, yz - sx)
    }

    /// Get front-direction vector
    ///
    /// Note: In OpenGL, "front" is often the -z direction.
    ///
    /// ```rust
    /// # use rmath::{Quaternion, Vector3};
    /// # use approx::assert_relative_eq;
    /// let q = Quaternion::one();
    /// assert_relative_eq!(q.front(), Vector3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn front(&self) -> Vector3 {
        let x2 = 2.0 * self.x * self.x;
        let y2 = 2.0 * self.y * self.y;
        let sx = 2.0 * self.s * self.x;
        let sy = 2.0 * self.s * self.y;
        let yz = 2.0 * self.y * self.z;
        let zx = 2.0 * self.z * self.x;
        Vector3::new(zx - sy, yz + sx, 1.0 - x2 - y2)
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

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vector3> for Quaternion {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let mat = Matrix::from(self);
        mat * rhs
    }
}

impl From<Quaternion> for Matrix {
    /// Convert the quaternion to rotate matrix
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
            [1.0 - y2 - z2, xy - sz, zx + sy, 0.0],
            [xy + sz, 1.0 - z2 - x2, yz - sx, 0.0],
            [zx - sy, yz + sx, 1.0 - x2 - y2, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Deg;
    use approx::assert_relative_eq;

    #[test]
    fn mul_synth() {
        let q1 = Quaternion::from_axis(Deg(45.0), Vector3::new(1.0, 0.0, 0.0));
        let q2 = Quaternion::from_axis(Deg(45.0), Vector3::new(0.0, 1.0, 0.0));
        // Turn first with q1, then with q2.
        let q = q1 * q2;
        assert_relative_eq!(
            q * Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(std::f64::consts::FRAC_1_SQRT_2, -0.5, 0.5)
        );
        // Turn first with q2, then with q1.
        let q = q2 * q1;
        assert_relative_eq!(
            q * Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.5, -std::f64::consts::FRAC_1_SQRT_2, 0.5)
        );
    }
}
