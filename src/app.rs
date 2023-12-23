use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use color::RGBA;
use rmath::{
    vec3, CutOffWorldLine, Deg, LineOscillateWorldLine, Matrix, Quaternion, StaticWorldLine,
    Vector3, WorldLine,
};

use crate::{
    backend::{Backend, JustLocalData, JustShader, LorentzLocalData, LorentzShader, Shader, Shape},
    key::KeyManager,
    player::Player,
};

fn wasm_error(s: String) -> JsValue {
    s.into()
}

pub struct InternalApp {
    backend: Backend,
    lorentz_shader: LorentzShader,
    just_shader: JustShader,
    arrow_shape: Shape,
    charge_shape: Shape,
    measurement_points: Vec<StaticWorldLine>,
    charge: Charge,
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
        let just_shader = JustShader::new(&backend)?;

        let num = 100;
        let mut measurement_points = Vec::new();
        for x in -num..=num {
            for y in -num..=num {
                measurement_points.push(StaticWorldLine::new(vec3(
                    x as f64 * 0.4,
                    y as f64 * 0.4,
                    0.1,
                )));
            }
        }
        let a = vec3(1.0, 0.0, 0.0);
        let wl = LineOscillateWorldLine::new(vec3(0.0, 0.0, 0.0), a, 0.1).unwrap();
        let charge = Charge {
            q: 1.0,
            world_line: CutOffWorldLine::new(wl, -200.0),
        };

        Ok(InternalApp {
            backend,
            lorentz_shader,
            just_shader,
            arrow_shape: shape::ArrowOption::new()
                .shaft_radius(0.01)
                .head_radius(0.04)
                .build()
                .into(),
            charge_shape: shape::IcosahedronOption::new().radius(0.1).build().into(),
            charge,
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
        let view_perspective =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0)
                * self.player.rot_matrix();
        let lorentz = self.player.lorentz_matrix();

        if let Some((charge_x, _, _)) = self
            .charge
            .world_line
            .past_intersection(self.player.position())
        {
            let charge_data = LorentzLocalData {
                color: RGBA::yellow(),
                lorentz,
                view_perspective,
                model: transition_matrix * Matrix::translation(charge_x.spatial()),
            };
            self.lorentz_shader
                .bind_shared_data(&self.backend, &self.charge_shape);
            self.lorentz_shader
                .draw(&self.backend, &self.charge_shape, &charge_data);
        }

        self.just_shader
            .bind_shared_data(&self.backend, &self.arrow_shape);
        for m in self.measurement_points.iter() {
            let (pos_on_player_plc, _, _) = m.past_intersection(self.player.position()).unwrap();
            let Some((charge_x, charge_u, charge_a)) =
                self.charge.world_line.past_intersection(pos_on_player_plc)
            else {
                continue;
            };
            let l = charge_x - pos_on_player_plc;
            let fs = Matrix::field_strength(self.charge.q, l.spatial(), charge_u, charge_a);
            let fs = lorentz * fs * lorentz.transposed();

            let pos = lorentz * (pos_on_player_plc - self.player.position());
            let me_factor = 10.0;
            let e = fs.field_strength_to_electric_field();
            let rotate = Matrix::from(Quaternion::from_rotation_arc(
                Vector3::Z_AXIS,
                e.normalized(),
            ));
            let length = Matrix::scale(vec3(1.0, 1.0, (1.0 + e.magnitude() * me_factor).log10()));
            let data = JustLocalData {
                color: RGBA::red(),
                model_view_perspective: view_perspective
                    * Matrix::translation(pos.spatial())
                    * rotate
                    * length,
            };
            self.just_shader
                .draw(&self.backend, &self.arrow_shape, &data);

            let m = fs.field_strength_to_magnetic_field();
            let rotate = Matrix::from(Quaternion::from_rotation_arc(
                Vector3::Z_AXIS,
                m.normalized(),
            ));
            let length = Matrix::scale(vec3(1.0, 1.0, (1.0 + m.magnitude() * me_factor).log10()));
            let data = JustLocalData {
                color: RGBA::blue(),
                model_view_perspective: view_perspective
                    * Matrix::translation(pos.spatial())
                    * rotate
                    * length,
            };
            self.just_shader
                .draw(&self.backend, &self.arrow_shape, &data);
        }
        self.backend.flush();

        Ok(())
    }
}
