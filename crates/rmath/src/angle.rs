use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad(pub f64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Deg(pub f64);

impl From<Deg> for Rad {
    fn from(deg: Deg) -> Self {
        Rad(deg.0 * std::f64::consts::PI / 180.0)
    }
}

impl From<Rad> for Deg {
    fn from(rad: Rad) -> Self {
        Deg(rad.0 * 180.0 / std::f64::consts::PI)
    }
}

impl AbsDiffEq for Rad {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

impl RelativeEq for Rad {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

impl AbsDiffEq for Deg {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

impl RelativeEq for Deg {
    fn default_max_relative() -> Self::Epsilon {
        f64::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn rad_to_deg() {
        assert_relative_eq!(Deg::from(Rad(std::f64::consts::PI)), Deg(180.0));
        assert_relative_eq!(Deg::from(Rad(std::f64::consts::FRAC_PI_2)), Deg(90.0));
        assert_relative_eq!(Deg::from(Rad(-std::f64::consts::FRAC_PI_4)), Deg(-45.0));
    }

    #[test]
    fn deg_to_rad() {
        assert_relative_eq!(Rad::from(Deg(180.0)), Rad(std::f64::consts::PI));
        assert_relative_eq!(Rad::from(Deg(-720.0)), Rad(-std::f64::consts::PI * 4.0));
    }
}
