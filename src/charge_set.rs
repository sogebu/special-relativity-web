use rand::{thread_rng, Rng};
use rmath::{
    vec3, vec4, DiscreteWorldLine, LineOscillateWorldLine, Matrix, PhaseSpace, StaticWorldLine,
    Vector3, Vector4, WorldLine,
};

#[derive(Copy, Clone)]
pub enum ChargePreset {
    Static,
    Eom,
    LineOscillate,
    LineOscillateEom,
    Dipole,
    Random,
}

impl std::str::FromStr for ChargePreset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "static" => Ok(ChargePreset::Static),
            "eom" => Ok(ChargePreset::Eom),
            "line_o" => Ok(ChargePreset::LineOscillate),
            "o_eom" => Ok(ChargePreset::LineOscillateEom),
            "dipole" => Ok(ChargePreset::Dipole),
            "random" => Ok(ChargePreset::Random),
            _ => Err(()),
        }
    }
}

pub trait ChargeSet {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))>;

    fn tick(&mut self, _c: f64, _until: Vector4) {}

    fn change_c(&mut self, _current_c: f64, _new_c: f64) {}

    fn info(&self, _c: f64, _s: &mut String, _player_pos: Vector4) {}
}

pub struct StaticChargeSet {
    charges: Vec<(f64, StaticWorldLine)>,
}

impl StaticChargeSet {
    pub fn new() -> StaticChargeSet {
        StaticChargeSet {
            charges: vec![(1.0, StaticWorldLine::new(vec3(0.0, 0.0, 0.0)))],
        }
    }
}

impl ChargeSet for StaticChargeSet {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        self.charges
            .iter()
            .map(|(q, wl)| (*q, wl.past_intersection(c, player_pos).unwrap()))
            .collect()
    }
}

pub struct EomCharge {
    q: f64,
    phase_space: PhaseSpace,
    world_line: DiscreteWorldLine,
}

pub struct EomChargeSet {
    charges: Vec<EomCharge>,
}

impl EomCharge {
    pub fn new(q: f64, x: Vector4, u: Vector3) -> EomCharge {
        let mut wl = DiscreteWorldLine::new();
        wl.push(Vector4::from_ctv(x.ct - 1e4, x.spatial()));
        wl.push(Vector4::from_ctv(x.ct - 1e3, x.spatial()));
        wl.push(Vector4::from_ctv(x.ct - 1e2, x.spatial()));
        wl.push(Vector4::from_ctv(x.ct - 1e1, x.spatial()));
        wl.push(Vector4::from_ctv(x.ct - 1e0, x.spatial()));
        wl.push(x);
        EomCharge {
            q,
            phase_space: PhaseSpace::new(u, x),
            world_line: wl,
        }
    }

    fn tick(&mut self, fs: Matrix, ds: f64) {
        let force =
            fs * (Matrix::eta() * Vector4::from_velocity(self.phase_space.velocity)) * self.q;
        self.phase_space.tick_in_world_frame(ds, force.spatial());
        self.world_line.push(self.phase_space.position);
    }
}

impl EomChargeSet {
    pub fn new_fixed_two_charges(c: f64, t: f64) -> EomChargeSet {
        let v = 0.5;
        let u = v / c;
        let r = 2.0;
        let c1 = EomCharge::new(-3.5, vec4(u * 2.0, r, 0.0, t), vec3(-u, 0.0, 0.0));
        let c2 = EomCharge::new(3.5, vec4(-u * 2.0, -r, 0.0, t), vec3(u, 0.0, 0.0));
        EomChargeSet {
            charges: vec![c1, c2],
        }
    }

    pub fn new_many_random_charges(c: f64, t: f64, n: usize) -> EomChargeSet {
        let mut charges = Vec::with_capacity(n);
        let mut rng = thread_rng();
        for i in 0..n {
            let l = 10.0_f64;
            let x = rng.gen_range(-l..l);
            let y = rng.gen_range(-l..l);
            let z = rng.gen_range(-l * 1e-2..l * 1e-2);
            let u = rng.gen_range(0f64..1.0 / c);
            let theta = x.atan2(y);
            let c = EomCharge::new(
                1.0 * if i % 2 == 0 { 1.0 } else { -1.0 },
                vec4(x, y, z, t),
                vec3(u * theta.cos(), u * theta.sin(), 0.0),
            );
            charges.push(c);
        }
        EomChargeSet { charges }
    }
}

impl ChargeSet for EomChargeSet {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        self.charges
            .iter()
            .filter_map(move |charge| {
                charge
                    .world_line
                    .past_intersection(c, player_pos)
                    .map(|x| (charge.q, x))
            })
            .collect()
    }

    fn tick(&mut self, c: f64, until: Vector4) {
        let ds = 1.0 / 100.0 * c;
        while !self.charges.iter().all(|charge| {
            charge.phase_space.position.ct >= until.ct
                || (charge.phase_space.position - until).lorentz_norm2() >= 0.0
        }) {
            let i = self
                .charges
                .iter()
                .enumerate()
                .min_by(|(_, ci), (_, cj)| {
                    ci.phase_space
                        .position
                        .ct
                        .total_cmp(&cj.phase_space.position.ct)
                })
                .map(|(i, _)| i)
                .unwrap();
            let position = self.charges[i].phase_space.position;
            let fs = field_strength_from_charges(c, &self.charges, i, position);
            self.charges[i].tick(fs, ds);
        }
    }

    fn change_c(&mut self, current_c: f64, new_c: f64) {
        for charge in self.charges.iter_mut() {
            charge.phase_space.change_c(current_c, new_c);
        }
    }

    fn info(&self, c: f64, s: &mut String, player_pos: Vector4) {
        for (i, charge) in self.charges.iter().enumerate() {
            let Some((x, u, _)) = charge.world_line.past_intersection(c, player_pos) else {
                continue;
            };
            s.push_str(&format!("charge {i} x = {}\n", x));
            s.push_str(&format!("charge {i} gamma = {:.3}\n", u.gamma()));
        }
    }
}

pub struct LineOscillateCharge {
    q: f64,
    world_line: LineOscillateWorldLine,
}

impl LineOscillateCharge {
    pub fn new(c: f64) -> LineOscillateCharge {
        LineOscillateCharge {
            q: 3.5,
            world_line: LineOscillateWorldLine::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(5.0 / std::f64::consts::TAU, 0.0, 0.0),
                (0.1 * c).min(0.4),
                c,
            )
            .unwrap(),
        }
    }
}

impl ChargeSet for LineOscillateCharge {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        self.world_line
            .past_intersection(c, player_pos)
            .into_iter()
            .map(|x| (self.q, x))
            .collect()
    }
}

pub struct DipoleCharge {
    q: f64,
    a: LineOscillateWorldLine,
    b: LineOscillateWorldLine,
}

impl DipoleCharge {
    pub fn new(c: f64) -> DipoleCharge {
        let f = (0.5 * c).min(5.0) / std::f64::consts::TAU;
        let x = Vector3::new(0.0, 1.2, 0.0);
        let v = Vector3::new(0.0, 1.0, 0.0);
        DipoleCharge {
            q: 3.5,
            a: LineOscillateWorldLine::new(x, v, f, c).unwrap(),
            b: LineOscillateWorldLine::new(-x, -v, f, c).unwrap(),
        }
    }
}

impl ChargeSet for DipoleCharge {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        let mut v = Vec::with_capacity(2);
        if let Some(x) = self.a.past_intersection(c, player_pos) {
            v.push((self.q, x));
        }
        if let Some(x) = self.b.past_intersection(c, player_pos) {
            v.push((-self.q, x));
        }
        v
    }
}

pub struct LineOscillateEomCharge {
    q: f64,
    world_line: LineOscillateWorldLine,
    charges: Vec<EomCharge>,
}

impl LineOscillateEomCharge {
    pub fn new(c: f64, t: f64) -> LineOscillateEomCharge {
        let r = 0.5;
        let v = 1.8;
        let u = v / c;
        let c1 = EomCharge::new(-3.5, vec4(0.0, r, 0.0, t), vec3(u, 0.0, 0.0));
        LineOscillateEomCharge {
            q: 3.5,
            world_line: LineOscillateWorldLine::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(5.0 / std::f64::consts::TAU, 0.0, 0.0),
                (0.1 * c).min(0.4),
                c,
            )
            .unwrap(),
            charges: vec![c1],
        }
    }
}

impl ChargeSet for LineOscillateEomCharge {
    fn iter(&self, c: f64, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        let mut v = self
            .world_line
            .past_intersection(c, player_pos)
            .into_iter()
            .map(|x| (self.q, x))
            .collect::<Vec<_>>();
        v.extend(self.charges.iter().filter_map(move |charge| {
            charge
                .world_line
                .past_intersection(c, player_pos)
                .map(|x| (charge.q, x))
        }));
        v
    }

    fn tick(&mut self, c: f64, until: Vector4) {
        let ds = 1.0 / 100.0 * c;
        while !self.charges.iter().all(|c| {
            c.phase_space.position.ct >= until.ct
                || (c.phase_space.position - until).lorentz_norm2() >= 0.0
        }) {
            let i = self
                .charges
                .iter()
                .enumerate()
                .min_by(|(_, ci), (_, cj)| {
                    ci.phase_space
                        .position
                        .ct
                        .total_cmp(&cj.phase_space.position.ct)
                })
                .map(|(i, _)| i)
                .unwrap();
            let position = self.charges[i].phase_space.position;
            let mut fs = field_strength_from_charges(c, &self.charges, i, position);
            if let Some((x, u, a)) = self.world_line.past_intersection(c, position) {
                fs =
                    fs + Matrix::field_strength(self.q / c, x.spatial() - position.spatial(), u, a);
            }
            self.charges[i].tick(fs, ds);
        }
    }

    fn change_c(&mut self, current_c: f64, new_c: f64) {
        for charge in self.charges.iter_mut() {
            charge.phase_space.change_c(current_c, new_c);
        }
    }
}

fn field_strength_from_charges(
    c: f64,
    charges: &[EomCharge],
    i: usize,
    position: Vector4,
) -> Matrix {
    let mut fs = Matrix::zero();
    for (j, charge) in charges.iter().enumerate() {
        // ignore form self
        if i == j {
            continue;
        }
        let Some((x, u, a)) = charge.world_line.past_intersection(c, position) else {
            continue;
        };
        fs = fs + Matrix::field_strength(charge.q / c, x.spatial() - position.spatial(), u, a);
    }
    fs
}
