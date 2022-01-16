use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext};
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use key::KeyManager;
use memoffset::offset_of;
use rmath::{Deg, Matrix, Quaternion, Rad, Vector3};

mod key;

fn wasm_error(s: String) -> JsValue {
    s.into()
}

#[allow(dead_code)]
fn log(s: String) {
    console::log_1(&s.into());
}

pub struct Backend {
    gl: Context,
    matrix_location: WebGlUniformLocation,
    triangle_count: usize,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vertex {
    position: Vector3,
    color: RGBA,
}

impl Backend {
    pub fn new(
        webgl2: WebGl2RenderingContext,
        vertices: &[Vertex],
        indices: &[[u32; 3]],
    ) -> Result<Self, String> {
        let gl = Context::from_webgl2_context(webgl2);
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.9, 0.9, 0.9, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            let program = gl.create_program()?;
            let fragment_shader_source = include_str!("fragment_shader.glsl");
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)?;
            gl.shader_source(vertex_shader, include_str!("vertex_shader.glsl"));
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                return Err(gl.get_shader_info_log(vertex_shader));
            }
            gl.attach_shader(program, vertex_shader);
            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)?;
            gl.shader_source(fragment_shader, fragment_shader_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                return Err(gl.get_shader_info_log(fragment_shader));
            }
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(gl.get_program_info_log(program));
            }
            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);
            gl.use_program(Some(program));

            let vbo = gl.create_buffer()?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_READ,
            );
            let ebo = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indices),
                glow::STATIC_READ,
            );

            let position_location = gl
                .get_attrib_location(program, "vert_position")
                .ok_or_else(|| "No vert_position attribute".to_string())?;
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_f32(
                position_location,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, position) as i32,
            );

            let color_location = gl
                .get_attrib_location(program, "vert_color")
                .ok_or_else(|| "No vert_color attribute".to_string())?;
            gl.enable_vertex_attrib_array(color_location);
            gl.vertex_attrib_pointer_f32(
                color_location,
                4,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, color) as i32,
            );

            let matrix_location = gl
                .get_uniform_location(program, "matrix")
                .ok_or_else(|| "No matrix attribute".to_string())?;

            Ok(Self {
                gl,
                matrix_location,
                triangle_count: indices.len(),
            })
        }
    }

    pub fn draw(&self, mat: Matrix) -> Result<(), String> {
        unsafe {
            self.gl
                .uniform_matrix_4_f32_slice(Some(&self.matrix_location), false, &mat.array());
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.draw_elements(
                glow::TRIANGLES,
                (self.triangle_count * 3) as i32,
                glow::UNSIGNED_INT,
                0,
            );
            self.gl.flush();
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub struct App {
    backend: Backend,
    key_manager: KeyManager,
    last_tick: Option<f64>,
    player: Player,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        let qube_vertices = [
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
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
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let num = 16;
        for x in 0..num {
            for y in 0..num {
                for z in 0..num {
                    let center = Vector3::new(x as f32, y as f32, z as f32) * 8.0;
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
                    for &v in qube_vertices.iter() {
                        vertices.push(Vertex {
                            position: center + v,
                            color,
                        });
                    }
                }
            }
        }
        Ok(App {
            backend: Backend::new(context, &vertices, &indices).map_err(wasm_error)?,
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

        let (width, height) = unsafe {
            let mut buf = [0; 4];
            self.backend
                .gl
                .get_parameter_i32_slice(glow::VIEWPORT, &mut buf);
            (buf[2] - buf[0], buf[3] - buf[1])
        };
        let projection_matrix =
            Matrix::perspective(Deg(60.0), width as f64 / height as f64, 0.1, 10000.0);
        let view_matrix = self.player.view_matrix();
        let mat = projection_matrix * view_matrix;
        self.backend.draw(mat).map_err(wasm_error)
    }
}

pub struct Player {
    pos: Vector3,
    quaternion: Quaternion,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector3::new(-9.0, -9.0, -30.0),
            quaternion: Quaternion::from_axis(Deg(130.0), Vector3::new(-1.0, 1.0, 0.0)),
        }
    }

    pub fn view_matrix(&self) -> Matrix {
        let rot = Matrix::from(self.quaternion);
        rot * Matrix::translation(-self.pos)
    }

    pub fn tick(&mut self, dt: f64, key: &KeyManager) {
        self.pos += self.get_velocity(dt, key);
        self.quaternion *= self.get_rotation_velocity(dt, key);
    }

    pub fn get_velocity(&self, dt: f64, key: &KeyManager) -> Vector3 {
        let mut d = Vector3::zero();
        // forward
        if key.is_pressed("w") {
            d -= self.quaternion.front()
        }
        if key.is_pressed("s") {
            d += self.quaternion.front()
        }
        // right-left
        if key.is_pressed("d") {
            d += self.quaternion.right();
        }
        if key.is_pressed("a") {
            d -= self.quaternion.right();
        }
        // up-down
        if key.is_pressed("z") {
            d -= self.quaternion.up();
        }
        if key.is_pressed("x") {
            d += self.quaternion.up();
        }
        d.safe_normalize() * dt as f32 * 20.0
    }

    pub fn get_rotation_velocity(&self, dt: f64, key: &KeyManager) -> Quaternion {
        let mut right = 0;
        if key.is_pressed("arrowright") {
            right += 1;
        }
        if key.is_pressed("arrowleft") {
            right -= 1;
        }
        let mut up = 0;
        if key.is_pressed("arrowup") {
            up += 1;
        }
        if key.is_pressed("arrowdown") {
            up -= 1;
        }
        let mut role = 0;
        if key.is_pressed("e") {
            role += 1;
        }
        if key.is_pressed("q") {
            role -= 1;
        }
        if (right, up, role) == (0, 0, 0) {
            Quaternion::one()
        } else {
            let axis = self.quaternion.up() * right as f32 - self.quaternion.right() * up as f32
                + self.quaternion.front() * role as f32;
            Quaternion::from_axis(Rad(dt), axis)
        }
    }
}
