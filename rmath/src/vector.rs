use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector3 {
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
    pub fn magnitude(self) -> f64 {
        self.magnitude2().sqrt()
    }

    /// Squared length of vector
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// let v = Vector3::new(1.0, 4.0, 8.0);
    /// assert_eq!(v.magnitude2(), 81.0);
    /// ```
    pub fn magnitude2(self) -> f64 {
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
    pub fn dot(self, rhs: Vector3) -> f64 {
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

    /// Get normalized vector
    ///
    /// panic: if magnitude of vector is zero
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// # use approx::assert_relative_eq;
    /// assert_relative_eq!(Vector3::new(3.0, 4.0, 0.0).normalized(), Vector3::new(0.6, 0.8, 0.0));
    /// ```
    pub fn normalized(self) -> Vector3 {
        self / self.magnitude()
    }

    /// Get normalized vector, but if vector magnitude equals zero
    /// then return zero vector.
    ///
    /// ```rust
    /// # use rmath::Vector3;
    /// # use approx::assert_relative_eq;
    /// assert_relative_eq!(Vector3::new(3.0, 0.0, 4.0).safe_normalized(), Vector3::new(0.6, 0.0, 0.8));
    /// assert_relative_eq!(Vector3::zero().safe_normalized(), Vector3::zero());
    /// ```
    pub fn safe_normalized(self) -> Vector3 {
        let magnitude = self.magnitude();
        if magnitude <= f64::EPSILON {
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

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, scaler: f64) -> Self::Output {
        Vector3::new(self.x * scaler, self.y * scaler, self.z * scaler)
    }
}
impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scaler: f64) {
        self.x *= scaler;
        self.y *= scaler;
        self.z *= scaler;
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, scaler: f64) -> Self::Output {
        Vector3::new(self.x / scaler, self.y / scaler, self.z / scaler)
    }
}
impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scaler: f64) {
        self.x /= scaler;
        self.y /= scaler;
        self.z /= scaler;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl AbsDiffEq for Vector3 {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl RelativeEq for Vector3 {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
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
