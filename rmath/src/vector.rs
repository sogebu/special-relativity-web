use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq};
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub const fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    /// Length of vector
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// let v = Vector3::new(1.0, 4.0, 8.0);
    /// assert_eq!(v.magnitude(), 9.0);
    /// ```
    pub fn magnitude(self) -> f32 {
        self.magnitude2().sqrt()
    }

    /// Squared length of vector
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// let v = Vector3::new(1.0, 4.0, 8.0);
    /// assert_eq!(v.magnitude2(), 81.0);
    /// ```
    pub fn magnitude2(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Dot product
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// let a = Vector3::new(2.0, 3.0, 4.0);
    /// let b = Vector3::new(0.5, 1.5, 2.5);
    /// assert_eq!(a.dot(b), 1.0 + 4.5 + 10.0);
    /// ```
    pub fn dot(self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Cross product
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// let a = Vector3::new(1.0, 2.0, 3.0);
    /// let b = Vector3::new(4.0, 5.0, 6.0);
    /// assert_eq!(a.cross(b), Vector3::new(-3.0, 6.0, -3.0));
    /// ```
    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - rhs.y * self.z,
            self.z * rhs.x - rhs.z * self.x,
            self.x * rhs.y - rhs.x * self.y,
        )
    }

    /// Normalize vector
    ///
    /// panic: if magnitude of vector is zero
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// # use approx::assert_relative_eq;
    /// assert_relative_eq!(Vector3::new(3.0, 4.0, 0.0).normalize(), Vector3::new(0.6, 0.8, 0.0));
    /// ```
    pub fn normalize(self) -> Vector3 {
        self / self.magnitude()
    }

    /// Normalize vector, but if vector magnitude equals zero
    /// then return zero vector.
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// # use approx::assert_relative_eq;
    /// assert_relative_eq!(Vector3::new(3.0, 0.0, 4.0).normalize(), Vector3::new(0.6, 0.0, 0.8));
    /// assert_relative_eq!(Vector3::zero(), Vector3::zero());
    /// ```
    pub fn safe_normalize(self) -> Vector3 {
        let magnitude = self.magnitude();
        if magnitude <= f32::EPSILON {
            Vector3::zero()
        } else {
            self / magnitude
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scaler: f32) -> Self::Output {
        Vector3::new(self.x * scaler, self.y * scaler, self.z * scaler)
    }
}
impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scaler: f32) {
        self.x *= scaler;
        self.y *= scaler;
        self.z *= scaler;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, scaler: f32) -> Self::Output {
        Vector3::new(self.x / scaler, self.y / scaler, self.z / scaler)
    }
}
impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, scaler: f32) {
        self.x /= scaler;
        self.y /= scaler;
        self.z /= scaler;
    }
}

impl AbsDiffEq for Vector3 {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl RelativeEq for Vector3 {
    fn default_max_relative() -> Self::Epsilon {
        f32::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.x.relative_eq(&other.x, epsilon, max_relative)
            && self.y.relative_eq(&other.y, epsilon, max_relative)
            && self.z.relative_eq(&other.z, epsilon, max_relative)
    }
}
