use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use color::RGBA;
use rmath::{
    vec3, CutOffWorldLine, Deg, LineOscillateWorldLine, Matrix, Quaternion, StaticWorldLine,
    Vector3, WorldLine,
};

use crate::{
    backend::{
        Backend, JustLocalData, LightingLocalData, LightingShader, LorentzLocalData, LorentzShader,
        Shader, Shape, SimpleShader, VertexPosition, VertexPositionNormal,
    },
    key::KeyManager,
    player::Player,
};

fn wasm_error(s: String) -> JsValue {
    s.into()
}

pub struct InternalApp {
    backend: Backend,
    lorentz_shader: LorentzShader,
    simple_shader: SimpleShader,
    lighting_shader: LightingShader,
    arrow_shape: Shape<VertexPosition>,
    arrow_config: ArrowConfig,
    charge_shape: Shape<VertexPosition>,
    arrow_shape2: Shape<VertexPositionNormal>,
    measurement_points: Vec<StaticWorldLine>,
    charges: Vec<Charge>,
    key_manager: KeyManager,
    last_tick: Option<f64>,
    player: Player,
}

struct Charge {
    q: f64,
    world_line: CutOffWorldLine<LineOscillateWorldLine>,
}

impl InternalApp {
    #[inline(always)]
    pub fn new(context: WebGl2RenderingContext) -> Result<InternalApp, JsValue> {
        let backend = Backend::new(context).map_err(wasm_error)?;
        let lorentz_shader = LorentzShader::new(&backend)?;
        let simple_shader = SimpleShader::new(&backend)?;
        let lighting_shader = LightingShader::new(&backend)?;

        let num = 50;
        let mut measurement_points = Vec::new();
        for x in -num..=num {
            for y in -num..=num {
                measurement_points.push(StaticWorldLine::new(vec3(
                    x as f64 * 0.5,
                    y as f64 * 0.5,
                    0.0,
                )));
            }
        }
        let wl =
            LineOscillateWorldLine::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), 0.1).unwrap();
        let charge1 = Charge {
            q: 1.0,
            world_line: CutOffWorldLine::new(wl, -200.0),
        };
        let wl = LineOscillateWorldLine::new(vec3(0., 2.0, 0.0), vec3(1.0, 0.0, 0.0), 0.1).unwrap();
        let charge2 = Charge {
            q: -1.0,
            world_line: CutOffWorldLine::new(wl, -200.0),
        };

        let arrow_config = ArrowConfig::default();

        Ok(InternalApp {
            backend,
            lorentz_shader,
            simple_shader,
            lighting_shader,
            arrow_shape: arrow_config.shape_data(),
            arrow_config,
            charge_shape: shape::IcosahedronOption::new()
                .radius(0.1)
                .build::<shape::VertexPosition>()
                .into(),
            arrow_shape2: shape::CubeOption::new()
                .build::<shape::VertexPositionCalcNormal>()
                .vertex_converted::<shape::VertexPositionNormal>()
                .into(),
            charges: vec![charge1, charge2],
            measurement_points,
            key_manager: KeyManager::new(),
            last_tick: None,
            player: Player::new(),
        })
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
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let last_tick = self.last_tick.replace(timestamp);
        let dt = (timestamp - last_tick.unwrap_or(timestamp)) / 1000.0;
        self.player.tick(dt, &self.key_manager);

        self.backend.clear();

        let (width, height) = self.backend.get_viewport_size();
        let transition_matrix = self.player.transition_matrix();
        let view_projection =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0)
                * self.player.rot_matrix();
        let lorentz = self.player.lorentz_matrix();

        for charge in self.charges.iter() {
            let Some((x, _, _)) = charge.world_line.past_intersection(self.player.position())
            else {
                continue;
            };
            let charge_data = LorentzLocalData {
                color: RGBA::yellow(),
                lorentz,
                view_projection,
                model: transition_matrix * Matrix::translation(x.spatial()),
            };
            self.lorentz_shader
                .bind_shared_data(&self.backend, &self.charge_shape);
            self.lorentz_shader
                .draw(&self.backend, &self.charge_shape, &charge_data);
        }

        self.simple_shader
            .bind_shared_data(&self.backend, &self.arrow_shape);
        for m in self.measurement_points.iter() {
            let (pos_on_player_plc, _, _) = m.past_intersection(self.player.position()).unwrap();

            let charges = self
                .charges
                .iter()
                .filter_map(|c| {
                    c.world_line
                        .past_intersection(pos_on_player_plc)
                        .map(|x| (c.q, x))
                })
                .collect::<Vec<_>>();
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
            let e = fs.field_strength_to_electric_field();
            self.draw_arrow(e, RGBA::red(), projection);
            let m = fs.field_strength_to_magnetic_field();
            self.draw_arrow(m, RGBA::blue(), projection);
        }

        self.lighting_shader
            .bind_shared_data(&self.backend, &self.arrow_shape2);
        let q = self.player.inv_rot_matrix();
        let data = LightingLocalData {
            color: RGBA::red(),
            model_view_projection: view_projection
                * transition_matrix
                * Matrix::translation(vec3(1.0, 1.0, -1.0))
                * Matrix::scale(vec3(1.0, 1.0, 1.0)),
            normal: q,
        };
        self.lighting_shader
            .draw(&self.backend, &self.arrow_shape2, &data);

        self.backend.flush();

        Ok(())
    }

    fn draw_arrow(&self, v: Vector3, color: RGBA, projection: Matrix) {
        let rotate = Matrix::from(Quaternion::from_rotation_arc(
            Vector3::Z_AXIS,
            v.normalized(),
        ));
        let data = JustLocalData {
            color,
            model_view_projection: projection
                * rotate
                * Matrix::scale(Vector3::new(1.0, 1.0, self.arrow_config.arrow_length(v))),
        };
        self.simple_shader
            .draw(&self.backend, &self.arrow_shape, &data);
    }
}

#[derive(Copy, Clone)]
pub struct ArrowConfig {
    shaft_radius: f32,
    head_radius: f32,
    log_count: u8,
    length_factor: f64,
}

impl Default for ArrowConfig {
    fn default() -> Self {
        ArrowConfig {
            shaft_radius: 0.01,
            head_radius: 0.04,
            log_count: 1,
            length_factor: 0.1,
        }
    }
}

impl ArrowConfig {
    pub fn shape_data(&self) -> Shape<VertexPosition> {
        shape::ArrowOption::new()
            .shaft_radius(self.shaft_radius)
            .head_radius(self.head_radius)
            .build::<shape::VertexPosition>()
            .dedup()
            .into()
    }

    pub fn arrow_length(&self, v: Vector3) -> f64 {
        let mut length = v.magnitude() * 1e3;
        for _ in 0..self.log_count {
            length = (1.0 + length).ln();
        }
        length * self.length_factor
    }
}
