use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub ct: f64,
}

impl Vector2 {
    pub const X_AXIS: Vector2 = Vector2::new(1.0, 0.0);
    pub const Y_AXIS: Vector2 = Vector2::new(0.0, 1.0);

    /// Length of vector
    ///
    /// ```rust
    /// # use rmath::Vector2;
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    pub fn magnitude(self) -> f64 {
        self.magnitude2().sqrt()
    }

    /// Squared length of vector
    ///
    /// ```rust
    /// # use rmath::Vector2;
    /// let v = Vector2::new(1.0, 4.0);
    /// assert_eq!(v.magnitude2(), 17.0);
    /// ```
    pub fn magnitude2(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Dot product
    ///
    /// ```rust
    /// # use rmath::Vector2;
    /// let a = Vector2::new(2.0, 3.0);
    /// let b = Vector2::new(0.5, 1.5);
    /// assert_eq!(a.dot(b), 1.0 + 4.5);
    /// ```
    pub fn dot(self, rhs: Vector2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Vector3 {
    pub const X_AXIS: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Vector3 = Vector3::new(0.0, 0.0, 1.0);

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

    /// Gamma factor
    pub fn gamma(&self) -> f64 {
        (1.0 + self.magnitude2()).sqrt()
    }
}

impl Vector4 {
    pub const fn from_ctv(ct: f64, v: Vector3) -> Vector4 {
        Vector4::new(v.x, v.y, v.z, ct)
    }

    /// Calculate Lorentz-squared-norm
    ///
    /// ```rust
    /// # use rmath::vec4;
    /// assert_eq!(vec4(1.0, 2.0, 3.0, 4.0).lorentz_norm2(), 1.0 + 4.0 + 9.0 - 16.0);
    /// ```
    pub fn lorentz_norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z - self.ct * self.ct
    }

    /// Calculate Lorentz-inner-product
    ///
    /// ```rust
    /// # use rmath::vec4;
    /// assert_eq!(
    ///     vec4(1.0, 2.0, 3.0, 4.0).lorentz_dot(vec4(2.0, 3.0, 4.0, 5.0)),
    ///     2.0 + 6.0 + 12.0 - 20.0,
    /// );
    /// ```
    pub fn lorentz_dot(&self, other: Vector4) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z - self.ct * other.ct
    }

    /// Get spatial vector
    pub const fn spatial(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Construct from spatial velocity vector
    ///
    /// The time-component of 4d velocity is gamma factor.
    pub fn from_velocity(u: Vector3) -> Vector4 {
        Vector4::new(u.x, u.y, u.z, u.gamma())
    }

    /// Construct from spatial acceleration vector
    ///
    /// The time-component if always zero.
    pub fn from_acceleration(a: Vector3) -> Vector4 {
        Vector4::new(a.x, a.y, a.z, 0.0)
    }
}

macro_rules! impl_vector {
    ($VectorN:ident { $($field:ident),+ }, $short:ident) => {
        impl $VectorN {
            pub const fn new($($field: f64),+) -> $VectorN {
                $VectorN { $($field),+ }
            }

            pub const fn zero() -> $VectorN {
                $VectorN { $($field: 0.0),+ }
            }
        }

        pub const fn $short($($field: f64),+) -> $VectorN {
            $VectorN { $($field),+ }
        }

        impl Add for $VectorN {
            type Output = $VectorN;
            fn add(self, rhs: Self) -> Self::Output {
                $VectorN::new($(self.$field + rhs.$field),+)
            }
        }
        impl AddAssign for $VectorN {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field);+
            }
        }

        impl Sub for $VectorN {
            type Output = $VectorN;
            fn sub(self, rhs: Self) -> Self::Output {
                $VectorN::new($(self.$field - rhs.$field),+)
            }
        }
        impl SubAssign for $VectorN {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field);+
            }
        }

        impl Mul<f64> for $VectorN {
            type Output = $VectorN;
            fn mul(self, rhs: f64) -> Self::Output {
                $VectorN::new($(self.$field * rhs),+)
            }
        }
        impl MulAssign<f64> for $VectorN {
            fn mul_assign(&mut self, rhs: f64) {
                $(self.$field *= rhs);+
            }
        }

        impl Div<f64> for $VectorN {
            type Output = $VectorN;
            fn div(self, rhs: f64) -> Self::Output {
                $VectorN::new($(self.$field / rhs),+)
            }
        }
        impl DivAssign<f64> for $VectorN {
            fn div_assign(&mut self, rhs: f64) {
                $(self.$field /= rhs);+
            }
        }

        impl Neg for $VectorN {
            type Output = $VectorN;
            fn neg(self) -> Self::Output {
                $VectorN::new($(-self.$field),+)
            }
        }

        impl AbsDiffEq for $VectorN  {
            type Epsilon = f64;
            fn default_epsilon() -> Self::Epsilon {
                f64::EPSILON
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                $(self.$field.abs_diff_eq(&other.$field, epsilon)) && +
            }
        }

        impl RelativeEq for $VectorN {
            fn default_max_relative() -> Self::Epsilon {
                f64::EPSILON
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                $(self.$field.relative_eq(&other.$field, epsilon, max_relative)) && +
            }
        }
    };
}

impl_vector!(Vector2 { x, y }, vec2);
impl_vector!(Vector3 { x, y, z }, vec3);
impl_vector!(Vector4 { x, y, z, ct }, vec4);

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Vector3::new(value[0] as f64, value[1] as f64, value[2] as f64)
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> Self {
        [value.x as f32, value.y as f32, value.z as f32]
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("({:.2}, {:.2}, {:.2})", self.x, self.y, self.z))
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "({:.2}; {:.2}, {:.2}, {:.2})",
            self.ct, self.x, self.y, self.z
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn add() {
        assert_relative_eq!(vec3(1.0, 2.0, 3.0) + Vector3::zero(), vec3(1.0, 2.0, 3.0));
        assert_relative_eq!(
            vec3(1.0, 2.0, 3.0) + vec3(4.0, 5.0, 6.0),
            vec3(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn sub() {
        assert_relative_eq!(vec3(1.0, 2.0, 3.0) - Vector3::zero(), vec3(1.0, 2.0, 3.0));
        assert_relative_eq!(
            vec3(1.0, 2.0, 3.0) - vec3(4.0, 6.0, 8.0),
            vec3(-3.0, -4.0, -5.0)
        );
    }

    #[test]
    fn mul() {
        assert_relative_eq!(vec3(1.0, 2.0, 3.0) * 0.0, Vector3::zero());
        assert_relative_eq!(vec3(5.0, 2.0, 3.0) * 4.0, vec3(20.0, 8.0, 12.0));
    }

    #[test]
    fn div() {
        assert_relative_eq!(vec3(5.0, 2.0, 3.0) / 4.0, vec3(1.25, 0.5, 0.75));
    }

    #[test]
    fn neg() {
        assert_eq!(-vec3(1.0, -2.0, 3.0), vec3(-1.0, 2.0, -3.0));
    }
}
