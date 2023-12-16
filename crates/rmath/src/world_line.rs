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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn static_world_line() {
        let wl = StaticWorldLine::new(Vector3::new(1.0, 2.0, 3.0));
        let (x, u, a) = wl.past_intersection(Vector4::from_tv(1.0, Vector3::new(-2.0, 2.0, -1.0)));
        assert_relative_eq!(x, Vector4::from_tv(-4.0, Vector3::new(1.0, 2.0, 3.0)));
        assert_eq!(u, Vector3::zero());
        assert_eq!(a, Vector3::zero());
    }
}
