use glow::Context;
use std::cmp::Ordering;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use backend::{Backend, LightingLocalData, LightingShader, Shader, Shape, VertexPositionNormal};
use color::RGBA;
use rmath::{vec3, Deg, Matrix, Quaternion, StaticWorldLine, Vector3, WorldLine};
use shape::BuildData;

use crate::charge_set::DipoleCharge;
use crate::{
    charge_set::{
        ChargePreset, ChargeSet, EomChargeSet, LineOscillateCharge, LineOscillateEomCharge,
        StaticChargeSet,
    },
    key::{GestureEvent, KeyManager, TouchManager},
    player::Player,
};

fn wasm_error(s: String) -> JsValue {
    s.into()
}

struct AppRender {
    backend: Backend<Context>,
    shader: LightingShader<Context>,
    arrow_shape: Shape<VertexPositionNormal>,
    charge_shape: Shape<VertexPositionNormal>,
}

struct AppInput {
    key_manager: KeyManager,
    touch_manager: TouchManager,
    last_tick: Option<f64>,
}

struct AppPhysics {
    /// Speed of Light
    c: f64,
    charge_preset: ChargePreset,
    charges: Box<dyn ChargeSet>,
    player: Player,
}

pub struct InternalApp {
    render: AppRender,
    input: AppInput,
    physics: AppPhysics,
    measurement_points: Vec<StaticWorldLine>,
    arrow_config: ArrowConfig,
    charge_scale: f64,
    poynting_on: bool,
}

impl AppRender {
    fn new(webgl2: WebGl2RenderingContext) -> Result<AppRender, JsValue> {
        let backend = Backend::new(Context::from_webgl2_context(webgl2)).map_err(wasm_error)?;
        let shader = LightingShader::new(&backend)?;

        let arrow_shape = shape::ArrowOption::new()
            .shaft_radius(0.02)
            .head_radius(0.02)
            .shaft_length(0.9)
            .head_length(0.1);
        Ok(AppRender {
            backend,
            shader,
            arrow_shape: arrow_shape.build_smooth().into(),
            charge_shape: shape::IcosahedronOption::new().build_sharp().into(),
        })
    }
}

impl AppInput {
    fn new(width: f64, height: f64) -> AppInput {
        AppInput {
            key_manager: KeyManager::new(),
            touch_manager: TouchManager::new(width, height),
            last_tick: None,
        }
    }

    fn gesture(&mut self, timestamp: f64) -> Vec<GestureEvent> {
        (0..4)
            .map_while(|_| self.touch_manager.consume_gesture(timestamp))
            .collect::<Vec<_>>()
    }
}

impl AppPhysics {
    fn new(c: f64, charge_preset: ChargePreset) -> AppPhysics {
        let (charges, player): (Box<dyn ChargeSet>, Player) = match charge_preset {
            ChargePreset::Static => (
                Box::new(StaticChargeSet::new()),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::Eom => (
                Box::new(EomChargeSet::new_fixed_two_charges(c, -30.0)),
                Player::new(Vector3::new(0.0, 0.0, 30.0)),
            ),
            ChargePreset::LineOscillate => (
                Box::new(LineOscillateCharge::new(c)),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::LineOscillateEom => (
                Box::new(LineOscillateEomCharge::new(c, -20.0)),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::Dipole => (
                Box::new(DipoleCharge::new(c)),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::Dipole2 => (
                Box::new(DipoleCharge::new_para(c)),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::Random => (
                Box::new(EomChargeSet::new_many_random_charges(c, -30.0, 10)),
                Player::new(Vector3::new(0.0, 0.0, 30.0)),
            ),
        };
        AppPhysics {
            c,
            charge_preset,
            charges,
            player,
        }
    }

    fn tick(&mut self, dt: f64, key: &KeyManager, gesture: &[GestureEvent]) {
        self.player.tick(self.c, dt, key, gesture);
        self.charges.tick(self.c, self.player.position());
    }
}

fn grid_surface_measurement_points() -> Vec<StaticWorldLine> {
    let num = 50;
    let mut grid = Vec::new();
    for x in -num..=num {
        let x = x as f64;
        for y in -num..=num {
            let y = y as f64;
            if y < -5.0 {
                continue;
            }
            grid.push(StaticWorldLine::new(Vector3::new(x, y, 0.0)));
        }
    }
    for x in -num..=num {
        let x = x as f64;
        for z in 1..=num {
            let z = z as f64;
            grid.push(StaticWorldLine::new(vec3(x, -5.0, z)));
        }
    }
    grid
}

fn grid_bulk_measurement_points() -> Vec<StaticWorldLine> {
    let num = 12;
    let mut grid = Vec::new();
    for x in -num..=num {
        for y in -num..=num {
            for z in -num..=num {
                grid.push(StaticWorldLine::new(vec3(x as f64, y as f64, z as f64)));
            }
        }
    }
    grid
}

impl InternalApp {
    #[inline(always)]
    pub fn new(webgl2: WebGl2RenderingContext) -> Result<InternalApp, JsValue> {
        let render = AppRender::new(webgl2)?;
        let (width, height) = render.backend.get_viewport_size();
        Ok(InternalApp {
            render,
            input: AppInput::new(width as f64, height as f64),
            physics: AppPhysics::new(1.0, ChargePreset::Static),
            measurement_points: grid_surface_measurement_points(),
            arrow_config: ArrowConfig::default(),
            charge_scale: 0.2,
            poynting_on: false,
        })
    }

    #[inline(always)]
    pub fn restart_physics(&mut self) {
        self.physics = AppPhysics::new(self.physics.c, self.physics.charge_preset);
    }

    #[inline(always)]
    pub fn change_c(&mut self, c: f64) -> bool {
        match self.physics.c.total_cmp(&c) {
            Ordering::Less => {
                self.physics.player.change_c(self.physics.c, c);
                self.physics.charges.change_c(self.physics.c, c);
                self.physics.c = c;
                false
            }
            Ordering::Equal => false,
            Ordering::Greater => {
                self.physics = AppPhysics::new(c, self.physics.charge_preset);
                true
            }
        }
    }

    #[inline(always)]
    pub fn reset_charge(&mut self, setup: &str) {
        self.physics = AppPhysics::new(self.physics.c, setup.parse().unwrap());
    }

    #[inline(always)]
    pub fn reset_grid(&mut self, setup: &str) {
        match setup {
            "2d" => {
                self.charge_scale = 0.2;
                self.measurement_points = grid_surface_measurement_points();
            }
            "3d" => {
                self.charge_scale = 0.3;
                self.measurement_points = grid_bulk_measurement_points();
            }
            _ => (),
        }
    }

    pub fn change_poynting_on(&mut self, poynting_on: bool) {
        self.poynting_on = poynting_on;
    }

    #[inline(always)]
    pub fn change_arrow_length_factor(&mut self, f: f64) {
        self.arrow_config.length_factor = f;
    }

    #[inline(always)]
    pub fn change_arrow_length_log(&mut self, log: u8) {
        self.arrow_config.log_count = log;
    }

    #[inline(always)]
    pub fn key_down(&mut self, key: String) {
        self.input.key_manager.down(key);
    }

    #[inline(always)]
    pub fn key_up(&mut self, key: String) {
        self.input.key_manager.up(key);
    }

    #[inline(always)]
    pub fn window_blue(&mut self) {
        self.input.key_manager.clear();
    }

    #[inline(always)]
    pub fn touch_start(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.input.touch_manager.touch_start(ms, x, y);
    }

    #[inline(always)]
    pub fn touch_move(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.input.touch_manager.touch_move(ms, x, y);
    }

    #[inline(always)]
    pub fn touch_end(&mut self, ms: f64) {
        self.input.touch_manager.touch_end(ms);
    }

    #[inline(always)]
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let last_tick = self.input.last_tick.replace(timestamp);
        let dt = (timestamp - last_tick.unwrap_or(timestamp)) / 1000.0;

        let gesture = self.input.gesture(timestamp);
        self.physics.tick(dt, &self.input.key_manager, &gesture);

        self.render.backend.clear();

        let c = self.physics.c;
        let (width, height) = self.render.backend.get_viewport_size();
        let view_projection =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0)
                * self.physics.player.rot_matrix();
        let lorentz = self.physics.player.lorentz_matrix();
        let normal = self.physics.player.inv_rot_matrix();
        let player_position = self.physics.player.position();

        self.render
            .shader
            .bind_shared_data(&self.render.backend, &self.render.charge_shape);
        let charge_scale = Matrix::uniform_scale(self.charge_scale);
        for (q, (x, _, _)) in self.physics.charges.iter(c, player_position) {
            let pos = lorentz * (x - player_position);
            let charge_data = LightingLocalData {
                color: if q > 0.0 { RGBA::red() } else { RGBA::blue() },
                model_view_projection: view_projection
                    * Matrix::translation(pos.spatial())
                    * charge_scale,
                normal,
            };
            self.render.shader.draw(
                &self.render.backend,
                &self.render.charge_shape,
                &charge_data,
            );
        }

        self.render
            .shader
            .bind_shared_data(&self.render.backend, &self.render.arrow_shape);
        for m in self.measurement_points.iter() {
            let (pos_on_player_plc, _, _) = m.past_intersection(c, player_position).unwrap();

            let charges = self.physics.charges.iter(c, pos_on_player_plc);
            if charges.is_empty() {
                continue;
            }
            let mut fs = Matrix::zero();
            for (q, (x, u, a)) in charges {
                let l = x - pos_on_player_plc;
                fs = fs + Matrix::field_strength(q / self.physics.c, l.spatial(), u, a);
            }
            fs = lorentz * fs * lorentz.transposed();

            let pos = lorentz * (pos_on_player_plc - player_position);
            let projection = view_projection * Matrix::translation(pos.spatial());
            let ele = fs.field_strength_to_electric_field(self.physics.c);
            if !self.poynting_on && ele.magnitude2() > 1e-16 {
                self.draw_arrow(ele, RGBA::green(), projection, normal);
            }
            let mag = fs.field_strength_to_magnetic_field();
            if !self.poynting_on && mag.magnitude2() > 1e-16 {
                self.draw_arrow(mag, RGBA::orange(), projection, normal);
            }
            if self.poynting_on {
                let poynting = ele.cross(mag) * c * c;
                if poynting.magnitude2() > 1e-16 {
                    self.draw_arrow(poynting, RGBA::hotpink(), projection, normal);
                }
            }
        }
        self.render.backend.flush();

        Ok(())
    }

    pub fn info(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("player x = {}\n", self.physics.player.position()));
        s.push_str(&format!("player u = {}\n", self.physics.player.velocity()));
        s.push_str(&format!(
            "player v = {}\n",
            self.physics.player.velocity() * self.physics.c
        ));
        s.push_str(&format!(
            "player gamma = {:.3}\n",
            self.physics.player.velocity().gamma()
        ));
        let c = self.physics.c;
        self.physics
            .charges
            .info(c, &mut s, self.physics.player.position());
        s
    }

    fn draw_arrow(&self, v: Vector3, color: RGBA, projection: Matrix, normal: Matrix) {
        let q = Quaternion::from_rotation_arc(Vector3::Z_AXIS, v.normalized());
        let rotate = Matrix::from(q);
        let model_view_projection = projection
            * rotate
            * Matrix::scale(Vector3::new(1.0, 1.0, self.arrow_config.arrow_length(v)));
        let data = LightingLocalData {
            color,
            model_view_projection,
            normal: normal * rotate,
        };
        self.render
            .shader
            .draw(&self.render.backend, &self.render.arrow_shape, &data);
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
            log_count: 1,
            length_factor: 1.0,
        }
    }
}

impl ArrowConfig {
    pub fn arrow_length(&self, v: Vector3) -> f64 {
        let mut length = v.magnitude() * self.length_factor;
        for _ in 0..self.log_count {
            length = (1.0 + length).ln();
        }
        length
    }
}
