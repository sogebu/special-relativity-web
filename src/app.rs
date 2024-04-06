use glow::Context;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use backend::{
    Backend, LightingLocalData, LightingShader, Shader, Shape, SimpleLocalData, SimpleShader,
    VertexPosition, VertexPositionNormal,
};
use color::RGBA;
use rmath::{vec3, Deg, Matrix, Quaternion, StaticWorldLine, Vector3, WorldLine};
use shape::BuildData;

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
    simple_shader: SimpleShader<Context>,
    lighting_shader: LightingShader<Context>,
    arrow_shape_no_normal: Shape<VertexPosition>,
    arrow_shape_with_normal: Shape<VertexPositionNormal>,
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
    lighting_on: bool,
}

impl AppRender {
    fn new(webgl2: WebGl2RenderingContext) -> Result<AppRender, JsValue> {
        let backend = Backend::new(Context::from_webgl2_context(webgl2)).map_err(wasm_error)?;
        let simple_shader = SimpleShader::new(&backend)?;
        let lighting_shader = LightingShader::new(&backend)?;

        let arrow_shape = shape::ArrowOption::new()
            .shaft_radius(0.02)
            .head_radius(0.02)
            .head_length(0.3);
        Ok(AppRender {
            backend,
            simple_shader,
            lighting_shader,
            arrow_shape_no_normal: arrow_shape.build_no_normal().into(),
            arrow_shape_with_normal: arrow_shape.build_smooth().into(),
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
                Box::new(EomChargeSet::new(c, -30.0)),
                Player::new(Vector3::new(0.0, 0.0, 30.0)),
            ),
            ChargePreset::LineOscillate => (
                Box::new(LineOscillateCharge::new()),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
            ),
            ChargePreset::LineOscillateEom => (
                Box::new(LineOscillateEomCharge::new(c, -20.0)),
                Player::new(Vector3::new(0.0, 0.0, 20.0)),
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

fn grid_bulk_measurement_points() -> Vec<StaticWorldLine> {
    let num = 12;
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
        let render = AppRender::new(webgl2)?;
        let (width, height) = render.backend.get_viewport_size();
        Ok(InternalApp {
            render,
            input: AppInput::new(width as f64, height as f64),
            physics: AppPhysics::new(1.0, ChargePreset::Static),
            measurement_points: grid_surface_measurement_points(),
            arrow_config: ArrowConfig::default(),
            charge_scale: 0.2,
            lighting_on: true,
        })
    }

    #[inline(always)]
    pub fn change_c(&mut self, c: f64) {
        self.physics = AppPhysics::new(c, self.physics.charge_preset);
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

        let (width, height) = self.render.backend.get_viewport_size();
        let view_projection =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0)
                * self.physics.player.rot_matrix();
        let lorentz = self.physics.player.lorentz_matrix();
        let normal = self.physics.player.inv_rot_matrix();
        let player_position = self.physics.player.position();

        self.render
            .lighting_shader
            .bind_shared_data(&self.render.backend, &self.render.charge_shape);
        let charge_scale = Matrix::scale(vec3(
            self.charge_scale,
            self.charge_scale,
            self.charge_scale,
        ));
        for (_, (x, _, _)) in self.physics.charges.iter(player_position) {
            let pos = lorentz * (x - player_position);
            let charge_data = LightingLocalData {
                color: RGBA::red(),
                model_view_projection: view_projection
                    * Matrix::translation(pos.spatial())
                    * charge_scale,
                normal,
            };
            self.render.lighting_shader.draw(
                &self.render.backend,
                &self.render.charge_shape,
                &charge_data,
            );
        }

        if self.lighting_on {
            self.render
                .lighting_shader
                .bind_shared_data(&self.render.backend, &self.render.arrow_shape_with_normal);
        } else {
            self.render
                .simple_shader
                .bind_shared_data(&self.render.backend, &self.render.arrow_shape_no_normal);
        }
        for m in self.measurement_points.iter() {
            let (pos_on_player_plc, _, _) = m.past_intersection(player_position).unwrap();

            let charges = self.physics.charges.iter(pos_on_player_plc);
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
            if ele.magnitude2() > 1e-16 {
                self.draw_arrow(ele, RGBA::green(), projection, normal);
            }
            let mag = fs.field_strength_to_magnetic_field();
            if mag.magnitude2() > 1e-16 {
                self.draw_arrow(mag, RGBA::orange(), projection, normal);
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
        self.physics
            .charges
            .info(&mut s, self.physics.player.position());
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
            self.render.lighting_shader.draw(
                &self.render.backend,
                &self.render.arrow_shape_with_normal,
                &data,
            );
        } else {
            let data = SimpleLocalData {
                color,
                model_view_projection,
            };
            self.render.simple_shader.draw(
                &self.render.backend,
                &self.render.arrow_shape_no_normal,
                &data,
            );
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
    pub fn arrow_length(&self, v: Vector3) -> f64 {
        let mut length = v.magnitude() * 1e3;
        for _ in 0..self.log_count {
            length = (1.0 + length).ln();
        }
        length * self.length_factor
    }
}
