use crate::{Matrix, Vector3, Vector4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseSpace {
    pub velocity: Vector3,
    pub position: Vector4,
}

impl PhaseSpace {
    /// Construct PhaseSpace instance
    pub const fn new(velocity: Vector3, position: Vector4) -> PhaseSpace {
        PhaseSpace { velocity, position }
    }

    /// Calculate the time evolution for one step based on
    /// the acceleration and the time tick width ``ds``
    /// given in the rest system.
    pub fn tick(&mut self, ds: f64, acceleration: Vector3) {
        let lorentz = Matrix::lorentz(-self.velocity);
        let acceleration = lorentz * Vector4::from_acceleration(acceleration);
        self.position += Vector4::from_velocity(self.velocity) * ds;
        self.velocity += acceleration.spatial() * ds;
    }

    pub fn tick_in_world_frame(&mut self, ds: f64, acceleration: Vector3) {
        self.position += Vector4::from_velocity(self.velocity) * ds;
        self.velocity += acceleration * ds;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn constant_speed() {
        let mut p1 = PhaseSpace::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector4::from_tv(0.0, Vector3::zero()),
        );
        let mut p2 = PhaseSpace::new(
            Vector3::new(1.0, 0.5, 0.0),
            Vector4::from_tv(0.0, Vector3::zero()),
        );
        for _ in 0..16 {
            p1.tick(1.0 / 8.0, Vector3::zero());
            p2.tick(1.0 / 8.0, Vector3::zero());
        }
        assert_relative_eq!(p1.velocity, Vector3::new(1.0, 2.0, 3.0));
        assert_relative_eq!(p1.position.spatial(), Vector3::new(2.0, 4.0, 6.0));
        assert_relative_eq!(
            p2.position,
            Vector4::from_tv(3.0, Vector3::new(2.0, 1.0, 0.0))
        );
        assert!(p1.position.t > p2.position.t);
    }
}
