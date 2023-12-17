use crate::{Vector3, Vector4};

pub trait WorldLine {
    /// x is observer's position
    fn past_intersection(&self, x: Vector4) -> (Vector4, Vector3, Vector3);
}

pub struct StaticWorldLine {
    /// in world frame
    pub pos: Vector3,
}

impl StaticWorldLine {
    pub fn new(pos: Vector3) -> StaticWorldLine {
        StaticWorldLine { pos }
    }
}

impl WorldLine for StaticWorldLine {
    fn past_intersection(&self, x: Vector4) -> (Vector4, Vector3, Vector3) {
        let t = x.t - (x.spatial() - self.pos).magnitude();
        (
            Vector4::from_tv(t, self.pos),
            Vector3::zero(),
            Vector3::zero(),
        )
    }
}

/// p(t) = center + amplitude * sin(ωt)
pub struct LineOscillateWorldLine {
    center: Vector3,
    amplitude: Vector3,
    omega: f64,
}

impl LineOscillateWorldLine {
    pub fn new(
        center: Vector3,
        amplitude: Vector3,
        frequency: f64,
    ) -> Result<LineOscillateWorldLine, ()> {
        let omega = frequency * std::f64::consts::TAU;
        if omega.abs() * amplitude.magnitude() > 1.0 {
            Err(())
        } else {
            Ok(LineOscillateWorldLine {
                center,
                amplitude,
                omega,
            })
        }
    }

    fn newton(&self, x: Vector4) -> f64 {
        let l = self.center - x.spatial();
        let l_len = l.magnitude();
        if l_len < f64::EPSILON * 2.0 {
            return x.t;
        }
        let mut t = x.t - l.magnitude() - self.amplitude.magnitude() * 2.0;
        for _ in 0..100 {
            let (sin, cos) = (self.omega * t).sin_cos();
            let amp = l + self.amplitude * sin;
            let f = (t - x.t) * (t - x.t) - amp.magnitude2();
            if f.abs() / l_len < 1e-12 {
                return t;
            }
            let fp = 2.0 * (t - x.t - amp.dot(self.amplitude) * self.omega * cos);
            t -= f / fp;
        }
        unreachable!()
    }
}

impl WorldLine for LineOscillateWorldLine {
    fn past_intersection(&self, x: Vector4) -> (Vector4, Vector3, Vector3) {
        let t = self.newton(x);
        let (sin, cos) = (self.omega * t).sin_cos();
        (
            Vector4::from_tv(t, self.center + self.amplitude * sin),
            self.amplitude * (self.omega * cos),
            self.amplitude * (-self.omega * self.omega * sin),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::Rng;
    use rand_pcg::Mcg128Xsl64;

    #[test]
    fn static_world_line() {
        let wl = StaticWorldLine::new(Vector3::new(1.0, 2.0, 3.0));
        let (x, u, a) = wl.past_intersection(Vector4::from_tv(1.0, Vector3::new(-2.0, 2.0, -1.0)));
        assert_relative_eq!(x, Vector4::from_tv(-4.0, Vector3::new(1.0, 2.0, 3.0)));
        assert_eq!(u, Vector3::zero());
        assert_eq!(a, Vector3::zero());
    }

    #[test]
    fn line_oscillate_world_line() {
        let mut rng = Mcg128Xsl64::new(1);
        for _ in 0..1000 {
            let x = rng.gen_range(-1000.0..1000.0);
            let y = rng.gen_range(-1000.0..1000.0);
            let z = rng.gen_range(-1000.0..1000.0);
            let ax = rng.gen_range(-1.0..1.0);
            let ay = rng.gen_range(-1.0..1.0);
            let az = rng.gen_range(-1.0..1.0);
            let amp = Vector3::new(ax, ay, az);
            let wl = LineOscillateWorldLine::new(
                Vector3::new(x, y, z),
                amp,
                rng.gen_range(0.0..1.0) / amp.magnitude() / std::f64::consts::TAU,
            )
            .unwrap();

            let x = Vector4::new(
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
            );
            let (px, _, _) = wl.past_intersection(x);
            assert!(
                (x - px).lorentz_norm2().abs() < 1e-8,
                "x={:?} px={:?} {}",
                x,
                px,
                (x - px).lorentz_norm2()
            );
        }
    }
}
