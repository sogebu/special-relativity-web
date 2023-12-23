use crate::{Vector3, Vector4};

pub trait WorldLine {
    /// x is observer's position
    fn past_intersection(&self, x: Vector4) -> Option<(Vector4, Vector3, Vector3)>;
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
    fn past_intersection(&self, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let t = x.t - (x.spatial() - self.pos).magnitude();
        Some((
            Vector4::from_tv(t, self.pos),
            Vector3::zero(),
            Vector3::zero(),
        ))
    }
}

pub struct CutOffWorldLine<W> {
    world_line: W,
    appeared: f64,
}

impl<W> CutOffWorldLine<W> {
    pub fn new(world_line: W, appeared: f64) -> CutOffWorldLine<W> {
        CutOffWorldLine {
            world_line,
            appeared,
        }
    }
}

impl<W: WorldLine> WorldLine for CutOffWorldLine<W> {
    fn past_intersection(&self, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let (x, u, a) = self.world_line.past_intersection(x)?;
        if x.t < self.appeared {
            None
        } else {
            Some((x, u, a))
        }
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
        for _ in 0..10 {
            let (sin, cos) = (self.omega * t).sin_cos();
            let amp = l + self.amplitude * sin;
            let f = (t - x.t) * (t - x.t) - amp.magnitude2();
            if f.abs() < 1e-12 * l_len {
                return t;
            }
            let fp = 2.0 * (t - x.t - amp.dot(self.amplitude) * self.omega * cos);
            if fp.abs() < 1e-8 {
                break;
            }
            t -= f / fp;
        }
        self.binary_search(x)
    }

    fn binary_search(&self, x: Vector4) -> f64 {
        let l = self.center - x.spatial();
        let f = |t: f64| {
            (t - x.t) * (t - x.t) - (l + self.amplitude * (self.omega * t).sin()).magnitude2()
        };
        let mut hi = x.t;
        let mut dt = 1.0;
        while f(hi - dt) < 0.0 {
            dt *= 2.0;
        }
        loop {
            dt *= 0.5;
            let mid = hi - dt;
            if mid == hi {
                return mid;
            }
            let y = f(mid);
            if y.abs() <= 1e-12 {
                return mid;
            }
            if y < 0.0 {
                hi = mid;
            }
        }
    }
}

impl WorldLine for LineOscillateWorldLine {
    fn past_intersection(&self, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let t = self.newton(x);
        let (sin, cos) = (self.omega * t).sin_cos();
        Some((
            Vector4::from_tv(t, self.center + self.amplitude * sin),
            self.amplitude * (self.omega * cos),
            self.amplitude * (-self.omega * self.omega * sin),
        ))
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
        let (x, u, a) = wl
            .past_intersection(Vector4::from_tv(1.0, Vector3::new(-2.0, 2.0, -1.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_tv(-4.0, Vector3::new(1.0, 2.0, 3.0)));
        assert_eq!(u, Vector3::zero());
        assert_eq!(a, Vector3::zero());
    }

    #[test]
    fn cut_off_world_line() {
        let wl = StaticWorldLine::new(Vector3::new(1.0, 2.0, 3.0));
        let wl = CutOffWorldLine::new(wl, -1.0);
        let (x, _, _) = wl
            .past_intersection(Vector4::from_tv(4.5, Vector3::new(-2.0, 2.0, -1.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_tv(-0.5, Vector3::new(1.0, 2.0, 3.0)));
        let p = wl.past_intersection(Vector4::from_tv(3.5, Vector3::new(-1.5, 2.0, -1.0)));
        assert!(p.is_none());
    }

    #[test]
    fn line_oscillate_world_line() {
        let mut rng = Mcg128Xsl64::new(1);
        for _ in 0..100000 {
            let x = rng.gen_range(-1000.0..1000.0);
            let y = rng.gen_range(-1000.0..1000.0);
            let z = rng.gen_range(-1000.0..1000.0);
            let center = Vector3::new(x, y, z);
            let ax = rng.gen_range(-1.0..1.0);
            let ay = rng.gen_range(-1.0..1.0);
            let az = rng.gen_range(-1.0..1.0);
            let amp = Vector3::new(ax, ay, az);
            let wl = LineOscillateWorldLine::new(
                center,
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
            let (px, pu, _) = wl.past_intersection(x).unwrap();
            assert!(
                (x - px).lorentz_norm2().abs() < 1e-8,
                "x={:?}\npx={:?}\nnorm={}\nu={}",
                x - Vector4::from_tv(0.0, center),
                px - Vector4::from_tv(0.0, center),
                (x - px).lorentz_norm2(),
                pu.magnitude(),
            );
        }
    }
}
