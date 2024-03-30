use glow::Context;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use backend::{
    Backend, LightingLocalData, LightingShader, Shader, Shape, SimpleLocalData, SimpleShader,
    VertexPosition, VertexPositionNormal,
};
use color::RGBA;
use rmath::{
    vec3, vec4, Deg, DiscreteWorldLine, LineOscillateWorldLine, Matrix, PhaseSpace, Quaternion,
    StaticWorldLine, Vector3, Vector4, WorldLine,
};
use shape::BuildData;

use crate::{
    key::{KeyManager, TouchManager},
    player::Player,
};

fn wasm_error(s: String) -> JsValue {
    s.into()
}

pub struct InternalApp {
    backend: Backend<Context>,
    simple_shader: SimpleShader<Context>,
    lighting_shader: LightingShader<Context>,
    arrow_shape_no_normal: Shape<VertexPosition>,
    arrow_shape_with_normal: Shape<VertexPositionNormal>,
    arrow_config: ArrowConfig,
    charge_shape: Shape<VertexPositionNormal>,
    measurement_points: Vec<StaticWorldLine>,
    grid: i32,
    charges: Box<dyn ChargeSet>,
    key_manager: KeyManager,
    touch_manager: TouchManager,
    last_tick: Option<f64>,
    player: Player,
    lighting_on: bool,
}

fn grid_measurement_points() -> Vec<StaticWorldLine> {
    let num = 50;
    let mut measurement_points = Vec::new();
    for x in -num..=num {
        for y in -num..=num {
            if y as f64 * 0.5 < -5.0 {
                continue;
            }
            measurement_points.push(StaticWorldLine::new(vec3(
                x as f64 * 0.5,
                y as f64 * 0.5,
                0.0,
            )));
        }
    }
    for x in -num..=num {
        for z in -num..=num {
            if z == 0 {
                continue;
            }
            if z as f64 * 0.5 < 0.0 {
                continue;
            }
            measurement_points.push(StaticWorldLine::new(vec3(
                x as f64 * 0.5,
                -5.0,
                z as f64 * 0.5,
            )));
        }
    }
    measurement_points
}

fn grid_3d_measurement_points() -> Vec<StaticWorldLine> {
    let num = 15;
    let mut measurement_points = Vec::new();
    for x in -num..=num {
        for y in -num..=num {
            for z in -num..=num {
                measurement_points.push(StaticWorldLine::new(vec3(
                    x as f64 * 0.5,
                    y as f64 * 0.5,
                    z as f64 * 0.5,
                )));
            }
        }
    }
    measurement_points
}

impl InternalApp {
    #[inline(always)]
    pub fn new(webgl2: WebGl2RenderingContext) -> Result<InternalApp, JsValue> {
        let backend = Backend::new(Context::from_webgl2_context(webgl2)).map_err(wasm_error)?;
        let simple_shader = SimpleShader::new(&backend)?;
        let lighting_shader = LightingShader::new(&backend)?;

        let (width, height) = backend.get_viewport_size();

        let arrow_config = ArrowConfig::default();
        Ok(InternalApp {
            backend,
            simple_shader,
            lighting_shader,
            arrow_shape_no_normal: arrow_config.shape_data().build_no_normal().into(),
            arrow_shape_with_normal: arrow_config.shape_data().build_smooth().into(),
            arrow_config,
            charge_shape: shape::IcosahedronOption::new()
                .radius(0.2)
                .build_sharp()
                .into(),
            charges: Box::new(LineOscillateEomCharge::new(-20.0)),
            measurement_points: grid_measurement_points(),
            grid: 2,
            key_manager: KeyManager::new(),
            touch_manager: TouchManager::new(width as f64, height as f64),
            last_tick: None,
            player: Player::new(Vector3::new(0.0, 0.0, 20.0)),
            lighting_on: true,
        })
    }

    #[inline(always)]
    pub fn reset_charge(&mut self, setup: &str) {
        match setup {
            "eom" => {
                self.player = Player::new(Vector3::new(0.0, 0.0, 30.0));
                self.charges = Box::new(EomChargeSet::new(-30.0));
            }
            "line_o" => {
                self.player = Player::new(Vector3::new(0.0, 0.0, 20.0));
                self.charges = Box::new(LineOscillateCharge::new());
            }
            "o_eom" => {
                self.player = Player::new(Vector3::new(0.0, 0.0, 20.0));
                self.charges = Box::new(LineOscillateEomCharge::new(-20.0));
            }
            _ => (),
        }
    }

    #[inline(always)]
    pub fn reset_grid(&mut self, setup: &str) {
        match setup {
            "2d" => {
                self.charge_shape = shape::IcosahedronOption::new()
                    .radius(0.2)
                    .build_sharp()
                    .into();
                self.measurement_points = grid_measurement_points();
                self.grid = 2;
            }
            "3d" => {
                self.charge_shape = shape::IcosahedronOption::new()
                    .radius(0.3)
                    .build_sharp()
                    .into();
                self.measurement_points = grid_3d_measurement_points();
                self.grid = 3;
            }
            _ => (),
        }
    }

    #[inline(always)]
    pub fn key_down(&mut self, key: String) {
        self.key_manager.down(key);
    }

    #[inline(always)]
    pub fn key_up(&mut self, key: String) {
        self.key_manager.up(key);
    }

    #[inline(always)]
    pub fn window_blue(&mut self) {
        self.key_manager.clear();
    }

    #[inline(always)]
    pub fn touch_start(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.touch_manager.touch_start(ms, x, y);
    }

    #[inline(always)]
    pub fn touch_move(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.touch_manager.touch_move(ms, x, y);
    }

    #[inline(always)]
    pub fn touch_end(&mut self, ms: f64) {
        self.touch_manager.touch_end(ms);
    }

    #[inline(always)]
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let last_tick = self.last_tick.replace(timestamp);
        let dt = (timestamp - last_tick.unwrap_or(timestamp)) / 1000.0;
        let gesture = (0..4)
            .map_while(|_| self.touch_manager.consume_gesture(timestamp))
            .collect::<Vec<_>>();
        self.player.tick(dt, &self.key_manager, &gesture);
        self.charges.tick(self.player.position());

        self.backend.clear();

        let (width, height) = self.backend.get_viewport_size();
        let view_projection =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0)
                * self.player.rot_matrix();
        let lorentz = self.player.lorentz_matrix();
        let normal = self.player.inv_rot_matrix();

        self.lighting_shader
            .bind_shared_data(&self.backend, &self.charge_shape);
        for (_, (x, _, _)) in self.charges.iter(self.player.position()) {
            let pos = lorentz * (x - self.player.position());
            let charge_data = LightingLocalData {
                color: RGBA::red(),
                model_view_projection: view_projection * Matrix::translation(pos.spatial()),
                normal,
            };
            self.lighting_shader
                .draw(&self.backend, &self.charge_shape, &charge_data);
        }

        if self.lighting_on {
            self.lighting_shader
                .bind_shared_data(&self.backend, &self.arrow_shape_with_normal);
        } else {
            self.simple_shader
                .bind_shared_data(&self.backend, &self.arrow_shape_no_normal);
        }
        for m in self.measurement_points.iter() {
            let (pos_on_player_plc, _, _) = m.past_intersection(self.player.position()).unwrap();

            let charges = self.charges.iter(pos_on_player_plc);
            if charges.is_empty() {
                continue;
            }
            let mut fs = Matrix::zero();
            for (q, (x, u, a)) in charges {
                let l = x - pos_on_player_plc;
                fs = fs + Matrix::field_strength(q, l.spatial(), u, a);
            }
            fs = lorentz * fs * lorentz.transposed();

            let pos = lorentz * (pos_on_player_plc - self.player.position());
            let projection = view_projection * Matrix::translation(pos.spatial());
            let ele = fs.field_strength_to_electric_field();
            if ele.magnitude2() > 1e-16 {
                self.draw_arrow(ele, RGBA::green(), projection, normal);
            }
            let mag = fs.field_strength_to_magnetic_field();
            if mag.magnitude2() > 1e-16 {
                self.draw_arrow(mag, RGBA::orange(), projection, normal);
            }
        }
        self.backend.flush();

        Ok(())
    }

    pub fn info(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("player x = {}\n", self.player.position()));
        s.push_str(&format!("player u = {}\n", self.player.velocity()));
        s.push_str(&format!(
            "player gamma = {:.3}\n",
            self.player.velocity().gamma()
        ));
        self.charges.info(&mut s, self.player.position());
        s
    }

    fn draw_arrow(&self, v: Vector3, color: RGBA, projection: Matrix, normal: Matrix) {
        let q = Quaternion::from_rotation_arc(Vector3::Z_AXIS, v.normalized());
        let rotate = Matrix::from(q);
        let model_view_projection = projection
            * rotate
            * Matrix::scale(Vector3::new(1.0, 1.0, self.arrow_config.arrow_length(v)));
        if self.lighting_on {
            let data = LightingLocalData {
                color,
                model_view_projection,
                normal: normal * rotate,
            };
            self.lighting_shader
                .draw(&self.backend, &self.arrow_shape_with_normal, &data);
        } else {
            let data = SimpleLocalData {
                color,
                model_view_projection,
            };
            self.simple_shader
                .draw(&self.backend, &self.arrow_shape_no_normal, &data);
        }
    }
}

#[derive(Copy, Clone)]
pub struct ArrowConfig {
    log_count: u8,
    length_factor: f64,
}

impl Default for ArrowConfig {
    fn default() -> Self {
        ArrowConfig {
            log_count: 2,
            length_factor: 0.1,
        }
    }
}

impl ArrowConfig {
    pub fn shape_data(&self) -> shape::ArrowOption {
        shape::ArrowOption::new()
            .shaft_radius(0.02)
            .head_radius(0.02)
            .head_length(0.3)
    }

    pub fn arrow_length(&self, v: Vector3) -> f64 {
        let mut length = v.magnitude() * 1e3;
        for _ in 0..self.log_count {
            length = (1.0 + length).ln();
        }
        length * self.length_factor
    }
}

trait ChargeSet {
    fn iter(&self, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))>;

    fn tick(&mut self, _until: Vector4) {}

    fn info(&self, _s: &mut String, _player_pos: Vector4) {}
}

struct EomCharge {
    q: f64,
    phase_space: PhaseSpace,
    world_line: DiscreteWorldLine,
}

struct EomChargeSet {
    charges: Vec<EomCharge>,
}

impl EomCharge {
    fn new(q: f64, x: Vector4, u: Vector3) -> EomCharge {
        let mut wl = DiscreteWorldLine::new();
        wl.push(Vector4::from_tv(x.t - 1e4, x.spatial()));
        wl.push(Vector4::from_tv(x.t - 1e3, x.spatial()));
        wl.push(Vector4::from_tv(x.t - 1e2, x.spatial()));
        wl.push(Vector4::from_tv(x.t - 1e1, x.spatial()));
        wl.push(Vector4::from_tv(x.t - 1e0, x.spatial()));
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
    fn new(t: f64) -> EomChargeSet {
        let u = 0.5;
        let r = 2.0;
        let c1 = EomCharge::new(-1.0, vec4(u * 2.0, r, 0.0, t), vec3(-u, 0.0, 0.0));
        let c2 = EomCharge::new(1.0, vec4(-u * 2.0, -r, 0.0, t), vec3(u, 0.0, 0.0));
        EomChargeSet {
            charges: vec![c1, c2],
        }
    }
}

impl ChargeSet for EomChargeSet {
    fn iter(&self, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        self.charges
            .iter()
            .filter_map(move |c| c.world_line.past_intersection(player_pos).map(|x| (c.q, x)))
            .collect()
    }

    fn tick(&mut self, until: Vector4) {
        let ds = 1.0 / 100.0;
        while !self.charges.iter().all(|c| {
            c.phase_space.position.t >= until.t
                || (c.phase_space.position - until).lorentz_norm2() >= 0.0
        }) {
            let i = self
                .charges
                .iter()
                .enumerate()
                .min_by(|(_, ci), (_, cj)| {
                    ci.phase_space
                        .position
                        .t
                        .total_cmp(&cj.phase_space.position.t)
                })
                .map(|(i, _)| i)
                .unwrap();
            let phase_space = self.charges[i].phase_space;
            let mut fs = Matrix::zero();
            for (j, c) in self.charges.iter().enumerate() {
                // ignore form self
                if i == j {
                    continue;
                }
                let Some((x, u, a)) = c.world_line.past_intersection(phase_space.position) else {
                    continue;
                };
                fs = fs
                    + Matrix::field_strength(
                        c.q,
                        x.spatial() - phase_space.position.spatial(),
                        u,
                        a,
                    );
            }
            self.charges[i].tick(fs, ds);
        }
    }

    fn info(&self, s: &mut String, player_pos: Vector4) {
        for (i, charge) in self.charges.iter().enumerate() {
            let Some((x, u, _)) = charge.world_line.past_intersection(player_pos) else {
                continue;
            };
            s.push_str(&format!("charge {i} x = {}\n", x));
            s.push_str(&format!("charge {i} gamma = {:.3}\n", u.gamma()));
        }
    }
}

struct LineOscillateCharge {
    q: f64,
    world_line: LineOscillateWorldLine,
}

impl LineOscillateCharge {
    pub fn new() -> LineOscillateCharge {
        LineOscillateCharge {
            q: 1.0,
            world_line: LineOscillateWorldLine::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(5.0 / std::f64::consts::TAU, 0.0, 0.0),
                0.1,
            )
            .unwrap(),
        }
    }
}

impl ChargeSet for LineOscillateCharge {
    fn iter(&self, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        self.world_line
            .past_intersection(player_pos)
            .into_iter()
            .map(|x| (self.q, x))
            .collect()
    }
}

struct LineOscillateEomCharge {
    q: f64,
    world_line: LineOscillateWorldLine,
    charges: Vec<EomCharge>,
}

impl LineOscillateEomCharge {
    fn new(t: f64) -> LineOscillateEomCharge {
        let r = 2.0;
        let c1 = EomCharge::new(-1.0, vec4(0.0, r, 0.0, t), vec3(0.8, 0.0, 0.0));
        LineOscillateEomCharge {
            q: 1.0,
            world_line: LineOscillateWorldLine::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(5.0 / std::f64::consts::TAU, 0.0, 0.0),
                0.1,
            )
            .unwrap(),
            charges: vec![c1],
        }
    }
}

impl ChargeSet for LineOscillateEomCharge {
    fn iter(&self, player_pos: Vector4) -> Vec<(f64, (Vector4, Vector3, Vector3))> {
        let mut v = self
            .world_line
            .past_intersection(player_pos)
            .into_iter()
            .map(|x| (self.q, x))
            .collect::<Vec<_>>();
        v.extend(
            self.charges
                .iter()
                .filter_map(move |c| c.world_line.past_intersection(player_pos).map(|x| (c.q, x))),
        );
        v
    }

    fn tick(&mut self, until: Vector4) {
        let ds = 1.0 / 100.0;
        while !self.charges.iter().all(|c| {
            c.phase_space.position.t >= until.t
                || (c.phase_space.position - until).lorentz_norm2() >= 0.0
        }) {
            let i = self
                .charges
                .iter()
                .enumerate()
                .min_by(|(_, ci), (_, cj)| {
                    ci.phase_space
                        .position
                        .t
                        .total_cmp(&cj.phase_space.position.t)
                })
                .map(|(i, _)| i)
                .unwrap();
            let phase_space = self.charges[i].phase_space;
            let mut fs = Matrix::zero();
            for (j, c) in self.charges.iter().enumerate() {
                // ignore form self
                if i == j {
                    continue;
                }
                let Some((x, u, a)) = c.world_line.past_intersection(phase_space.position) else {
                    continue;
                };
                fs = fs
                    + Matrix::field_strength(
                        c.q,
                        x.spatial() - phase_space.position.spatial(),
                        u,
                        a,
                    );
            }

            if let Some((x, u, a)) = self.world_line.past_intersection(phase_space.position) {
                fs = fs
                    + Matrix::field_strength(
                        self.q,
                        x.spatial() - phase_space.position.spatial(),
                        u,
                        a,
                    );
            }
            self.charges[i].tick(fs, ds);
        }
    }
}
