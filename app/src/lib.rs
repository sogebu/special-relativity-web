use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use color::RGBA;
use rmath::{vec3, Deg, Matrix, Quaternion, Vector3};

use crate::{
    backend::{Backend, Entity, Vertex},
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
    entities: Vec<Entity>,
    entity_prop: Vec<EntityProp>,
    key_manager: KeyManager,
    last_tick: Option<f64>,
    player: Player,
}

pub struct EntityProp {
    world_pos: Vector3,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        let mut rng = Mcg128Xsl64::new(3);
        let qube_vertices = [
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
        ];
        let qube_indices = [
            [0, 1, 2],
            [0, 2, 3],
            [0, 5, 1],
            [0, 4, 5],
            [0, 7, 4],
            [0, 3, 7],
            [6, 1, 5],
            [6, 2, 1],
            [6, 5, 4],
            [6, 4, 7],
            [6, 7, 3],
            [6, 3, 2],
        ];
        let num = 10;
        let d = 8.0;

        let backend = Backend::new(context).map_err(wasm_error)?;
        let mut entities = Vec::new();
        let mut entity_prop = Vec::new();
        for x in 0..num {
            for y in 0..num {
                for z in 0..num {
                    let mut vertices = Vec::new();
                    let mut indices = Vec::new();
                    let color = RGBA::new(
                        ((x * (256 / num)) as f32) / 255.0,
                        ((y * (256 / num)) as f32) / 255.0,
                        ((z * (256 / num)) as f32) / 255.0,
                        1.0,
                    );
                    let vertex_num = vertices.len() as u32;
                    for &i in qube_indices.iter() {
                        indices.push([vertex_num + i[0], vertex_num + i[1], vertex_num + i[2]]);
                    }
                    let wx = rng.gen_range(-d * num as f64..d * num as f64);
                    let wy = rng.gen_range(-d * num as f64..d * num as f64);
                    let wz = rng.gen_range(-d * num as f64..d * num as f64);
                    entity_prop.push(EntityProp {
                        world_pos: vec3(wx, wy, wz),
                    });

                    for &v in qube_vertices.iter() {
                        vertices.push(Vertex {
                            local_position: v,
                            color,
                        });
                    }
                    entities.push(backend.new_entity(&vertices, &indices)?);
                }
            }
        }

        Ok(App {
            backend,
            entities,
            entity_prop,
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

        let rotate = Quaternion::from_axis(Deg(timestamp * 0.01), vec3(0.0, 0.0, 1.0));
        for (ent, prop) in self.entities.iter_mut().zip(self.entity_prop.iter()) {
            ent.model_local_matrix = Matrix::translation(prop.world_pos)
                * Matrix::from(rotate)
                * Matrix::scale(vec3(10.0, 1.0, 1.0));
        }

        let (width, height) = self.backend.get_viewport_size();
        let projection_matrix =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0);
        let rot_matrix = self.player.rot_matrix();
        let transition_matrix = self.player.transition_matrix();
        let lorentz_matrix = self.player.lorentz_matrix();
        self.backend
            .draw(
                &self.entities,
                transition_matrix,
                lorentz_matrix,
                projection_matrix * rot_matrix,
            )
            .map_err(wasm_error)
    }
}
