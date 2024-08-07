use crate::{Vector3, Vector4};

pub trait WorldLine {
    /// x is observer's position
    fn past_intersection(&self, c: f64, x: Vector4) -> Option<(Vector4, Vector3, Vector3)>;
}

#[derive(Debug, Clone)]
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
    fn past_intersection(&self, _c: f64, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let t = x.ct - (x.spatial() - self.pos).magnitude();
        Some((
            Vector4::from_ctv(t, self.pos),
            Vector3::zero(),
            Vector3::zero(),
        ))
    }
}

#[derive(Debug, Clone)]
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
    fn past_intersection(&self, c: f64, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let (x, u, a) = self.world_line.past_intersection(c, x)?;
        if x.ct < self.appeared {
            None
        } else {
            Some((x, u, a))
        }
    }
}

/// x_0 = ct
/// p(x_0) = center + amplitude * sin(ω x_0 / c)
/// p'(x_0) = amplitude * ω / c * cos(ω x_0 / c)
#[derive(Debug, Clone)]
pub struct LineOscillateWorldLine {
    center: Vector3,
    amplitude: Vector3,
    omega: f64,
}

impl LineOscillateWorldLine {
    #[allow(clippy::result_unit_err)]
    pub fn new(
        center: Vector3,
        amplitude: Vector3,
        frequency: f64,
        c: f64,
    ) -> Result<LineOscillateWorldLine, ()> {
        let omega = frequency * std::f64::consts::TAU;
        if omega.abs() * amplitude.magnitude() > c {
            Err(())
        } else {
            Ok(LineOscillateWorldLine {
                center,
                amplitude,
                omega,
            })
        }
    }

    fn newton(&self, c: f64, x: Vector4) -> f64 {
        let l = self.center - x.spatial();
        let l_len = l.magnitude();
        if l_len < f64::EPSILON * 2.0 {
            return x.ct;
        }
        let mut ct = x.ct - l.magnitude() - self.amplitude.magnitude() * 2.0;
        for _ in 0..10 {
            let (sin, cos) = (self.omega * ct / c).sin_cos();
            let amp = l + self.amplitude * sin;
            let f = (ct - x.ct) * (ct - x.ct) - amp.magnitude2();
            if f.abs() < 1e-12 * l_len {
                return ct;
            }
            let fp = 2.0 * (ct - x.ct - amp.dot(self.amplitude) * self.omega * cos);
            if fp.abs() < 1e-8 {
                break;
            }
            ct -= f / fp;
        }
        self.binary_search(c, x)
    }

    fn binary_search(&self, c: f64, x: Vector4) -> f64 {
        let l = self.center - x.spatial();
        let f = |t: f64| {
            (t - x.ct) * (t - x.ct) - (l + self.amplitude * (self.omega * t / c).sin()).magnitude2()
        };
        let mut hi = x.ct;
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
    fn past_intersection(&self, c: f64, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let ct = self.newton(c, x);
        let (sin, cos) = (self.omega * ct / c).sin_cos();
        Some((
            Vector4::from_ctv(ct, self.center + self.amplitude * sin),
            self.amplitude * (self.omega * cos),
            self.amplitude * (-self.omega * self.omega * sin),
        ))
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiscreteWorldLine {
    x: Vec<Vector4>,
}

impl DiscreteWorldLine {
    pub fn new() -> DiscreteWorldLine {
        DiscreteWorldLine { x: Vec::new() }
    }

    pub fn push(&mut self, x: Vector4) {
        self.x.push(x);
    }

    pub fn last(&self) -> Option<Vector4> {
        self.x.last().copied()
    }

    fn find_future_nearest(&self, x: Vector4) -> Option<usize> {
        if self.x.len() <= 2 {
            return None;
        }
        // lo = past = norm is negative
        // most post point is space-like = ng
        let mut lo = 1;
        let norm_lo = (self.x[lo] - x).lorentz_norm2();
        if norm_lo > 0.0 {
            return None;
        }
        // hi = future = norm is positive
        let mut hi = self.x.len() - 1;
        if self.x[hi].ct < x.ct && (self.x[hi] - x).lorentz_norm2() < 0.0 {
            return None;
        }
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.x[mid].ct >= x.ct || (self.x[mid] - x).lorentz_norm2() >= 0.0 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        Some(hi)
    }
}

impl WorldLine for DiscreteWorldLine {
    fn past_intersection(&self, _c: f64, x: Vector4) -> Option<(Vector4, Vector3, Vector3)> {
        let i = self.find_future_nearest(x)?;
        let x0 = self.x[i - 2]; // next to nearest past
        let x1 = self.x[i - 1]; // nearest past
        let x2 = self.x[i]; // most future

        let a = -(x2 - x1).lorentz_norm2();
        let b = -(x2 - x1).lorentz_dot(x - x1);
        let c = -(x - x1).lorentz_norm2();
        let lambda = c / (b + (b * b - a * c).sqrt());

        let tau0 = (-(x1 - x0).lorentz_norm2()).sqrt();
        let tau1 = (-(x2 - x1).lorentz_norm2()).sqrt();
        let u0 = (x1 - x0).spatial() / tau0;
        let u1 = (x2 - x1).spatial() / tau1;
        let acc = (u1 - u0) * (2.0 / (tau0 + tau1));

        Some((x1 * (1.0 - lambda) + x2 * lambda, u1, acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::Rng;
    use rand_pcg::Mcg128Xsl64;

    #[test]
    fn discrete_world_line_zero() {
        use crate::PhaseSpace;
        let mut wl = DiscreteWorldLine::new();
        let mut p = PhaseSpace::new(Vector3::zero(), Vector4::zero());
        for _ in 0..100 {
            wl.push(p.position);
            p.tick(0.5, Vector3::zero());
        }
        let (x, u, a) = wl
            .past_intersection(1.0, Vector4::from_ctv(2.0, Vector3::new(1.0, 0.0, 0.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_ctv(1.0, Vector3::zero()));
        assert_relative_eq!(u, Vector3::zero());
        assert_relative_eq!(a, Vector3::zero());
        let (x, u, a) = wl
            .past_intersection(1.0, Vector4::from_ctv(2.25, Vector3::new(1.0, 0.0, 0.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_ctv(1.25, Vector3::zero()));
        assert_relative_eq!(u, Vector3::zero());
        assert_relative_eq!(a, Vector3::zero());
    }

    #[test]
    fn discrete_world_line_exp() {
        let mut wl = DiscreteWorldLine::new();
        let x = Vector3::new(1.0, 2.0, 3.0);
        wl.push(Vector4::from_ctv(-1e4, x));
        wl.push(Vector4::from_ctv(-1e3, x));
        wl.push(Vector4::from_ctv(-1e2, x));
        wl.push(Vector4::from_ctv(-1e1, x));
        wl.push(Vector4::from_ctv(-1e0, x));
        assert_relative_eq!(
            wl.past_intersection(1.0, Vector4::new(0.0, 0.0, 0.0, 0.0))
                .unwrap()
                .0,
            Vector4::new(1.0, 2.0, 3.0, -(1.0f64 + 4.0 + 9.0).sqrt())
        );
        assert_relative_eq!(
            wl.past_intersection(1.0, Vector4::new(30.0, 0.0, 0.0, -30.0))
                .unwrap()
                .0,
            Vector4::new(1.0, 2.0, 3.0, -30.0 - (29.0 * 29.0 + 4.0 + 9.0f64).sqrt())
        );
    }

    #[test]
    fn discrete_world_line_cut_off() {
        let mut wl = DiscreteWorldLine::new();
        let x = Vector3::new(3.0, 0.0, 4.0);
        wl.push(Vector4::from_ctv(-1.0, x));
        wl.push(Vector4::from_ctv(0.0, x));
        wl.push(Vector4::from_ctv(1.0, x));
        wl.push(Vector4::from_ctv(2.0, x));
        assert_eq!(
            wl.past_intersection(1.0, Vector4::new(0.0, 0.0, 0.0, 4.9999)),
            None
        );
        assert_eq!(
            wl.past_intersection(1.0, Vector4::new(0.0, 0.0, 0.0, 5.0009765625))
                .unwrap()
                .0,
            Vector4::from_ctv(0.0009765625, x),
        );
        assert_relative_eq!(
            wl.past_intersection(1.0, Vector4::new(0.0, 0.0, 0.0, 6.9999))
                .unwrap()
                .0,
            Vector4::from_ctv(1.9999, x),
        );
        assert_eq!(
            wl.past_intersection(1.0, Vector4::new(0.0, 0.0, 0.0, 7.0001)),
            None
        );
    }

    #[test]
    fn static_world_line() {
        let wl = StaticWorldLine::new(Vector3::new(1.0, 2.0, 3.0));
        let (x, u, a) = wl
            .past_intersection(1.0, Vector4::from_ctv(1.0, Vector3::new(-2.0, 2.0, -1.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_ctv(-4.0, Vector3::new(1.0, 2.0, 3.0)));
        assert_eq!(u, Vector3::zero());
        assert_eq!(a, Vector3::zero());
    }

    #[test]
    fn cut_off_world_line() {
        let wl = StaticWorldLine::new(Vector3::new(1.0, 2.0, 3.0));
        let wl = CutOffWorldLine::new(wl, -1.0);
        let (x, _, _) = wl
            .past_intersection(1.0, Vector4::from_ctv(4.5, Vector3::new(-2.0, 2.0, -1.0)))
            .unwrap();
        assert_relative_eq!(x, Vector4::from_ctv(-0.5, Vector3::new(1.0, 2.0, 3.0)));
        let p = wl.past_intersection(1.0, Vector4::from_ctv(3.5, Vector3::new(-1.5, 2.0, -1.0)));
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
                1.0,
            )
            .unwrap();

            let x = Vector4::new(
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
                rng.gen_range(-1000.0..1000.0),
            );
            let (px, pu, _) = wl.past_intersection(1.0, x).unwrap();
            assert!(
                (x - px).lorentz_norm2().abs() < 1e-8,
                "x={:?}\npx={:?}\nnorm={}\nu={}",
                x - Vector4::from_ctv(0.0, center),
                px - Vector4::from_ctv(0.0, center),
                (x - px).lorentz_norm2(),
                pu.magnitude(),
            );
        }
    }
}
