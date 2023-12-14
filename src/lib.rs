use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use color::RGBA;
use rmath::{vec3, Deg, Matrix, Quaternion, StaticWorldLine, WorldLine};

use crate::{
    backend::{Backend, LorentzLocalData, LorentzShader, Shader, Shape},
    key::KeyManager,
    player::Player,
};

mod backend;
mod key;
mod player;

fn wasm_error(s: String) -> JsValue {
    s.into()
}

#[allow(dead_code)]
fn log(s: String) {
    console::log_1(&s.into());
}

#[wasm_bindgen]
pub struct App {
    backend: Backend,
    lorentz_shader: LorentzShader,
    qube: Shape,
    qube_properties: Vec<QubeProperty>,
    key_manager: KeyManager,
    last_tick: Option<f64>,
    player: Player,
}

pub struct QubeProperty {
    color: RGBA,
    world_line: StaticWorldLine,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        let mut rng = Mcg128Xsl64::new(3);

        let backend = Backend::new(context).map_err(wasm_error)?;
        let lorentz_shader = LorentzShader::new(&backend)?;

        let num = 20;
        let mut qube_properties = Vec::new();
        for x in 0..num {
            for y in 0..num {
                for z in 0..num {
                    let color = RGBA::new(
                        ((x * (256 / num)) as f32) / 255.0,
                        ((y * (256 / num)) as f32) / 255.0,
                        ((z * (256 / num)) as f32) / 255.0,
                        1.0,
                    );
                    let d = 15.0;
                    let wx = rng.gen_range(-d * num as f64..d * num as f64);
                    let wy = rng.gen_range(-d * num as f64..d * num as f64);
                    let wz = rng.gen_range(-d * num as f64..d * num as f64);
                    qube_properties.push(QubeProperty {
                        color,
                        world_line: StaticWorldLine::new(vec3(wx, wy, wz)),
                    });
                }
            }
        }

        Ok(App {
            backend,
            lorentz_shader,
            qube: Shape::qube(),
            qube_properties,
            key_manager: KeyManager::new(),
            last_tick: None,
            player: Player::new(),
        })
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: String) {
        self.key_manager.down(key);
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self, key: String) {
        self.key_manager.up(key);
    }

    #[wasm_bindgen]
    pub fn window_blue(&mut self) {
        self.key_manager.clear();
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let last_tick = self.last_tick.replace(timestamp);
        let dt = (timestamp - last_tick.unwrap_or(timestamp)) / 1000.0;
        self.player.tick(dt, &self.key_manager);

        self.backend.clear();

        self.lorentz_shader
            .bind_shared_data(&self.backend, &self.qube);

        let (width, height) = self.backend.get_viewport_size();
        let transition_matrix = self.player.transition_matrix();
        let projection_matrix =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0);
        let rot_matrix = self.player.rot_matrix();
        let lorentz_matrix = self.player.lorentz_matrix();

        for prop in self.qube_properties.iter() {
            let (pos_in_plc, _, _) = prop.world_line.past_intersection(self.player.position());
            let rotate = Quaternion::from_axis(Deg(pos_in_plc.t * 100.0), vec3(0.0, 0.0, 1.0));
            let model_local_matrix = Matrix::translation(pos_in_plc.spatial())
                * Matrix::from(rotate)
                * Matrix::scale(vec3(10.0, 1.0, 1.0));
            let data = LorentzLocalData {
                color: prop.color,
                model: transition_matrix * model_local_matrix,
                lorentz: lorentz_matrix,
                view_perspective: projection_matrix * rot_matrix,
            };
            self.lorentz_shader.draw(&self.backend, &self.qube, &data);
        }
        self.backend.flush();

        Ok(())
    }
}
