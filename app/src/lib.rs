use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext, WebBufferKey};
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use key::KeyManager;
use memoffset::offset_of;
use rmath::{vec3, Deg, Matrix, PhaseSpace, Quaternion, Rad, Vector3, Vector4};

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
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    model_matrix_location: WebGlUniformLocation,
    lorentz_matrix_location: WebGlUniformLocation,
    view_perspective_location: WebGlUniformLocation,
    triangle_count: usize,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vertex {
    local_position: [f32; 3],
    world_position: [f32; 3],
    scale: [f32; 3],
    color: RGBA,
}

impl Backend {
    pub fn new(
        webgl2: WebGl2RenderingContext,
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
            let ebo = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indices),
                glow::STATIC_READ,
            );

            let loc = gl
                .get_attrib_location(program, "vert_local_position")
                .ok_or_else(|| "No vert_local_position attribute".to_string())?;
            gl.enable_vertex_attrib_array(loc);
            gl.vertex_attrib_pointer_f32(
                loc,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, local_position) as i32,
            );
            let loc = gl
                .get_attrib_location(program, "vert_world_position")
                .ok_or_else(|| "No vert_world_position attribute".to_string())?;
            gl.enable_vertex_attrib_array(loc);
            gl.vertex_attrib_pointer_f32(
                loc,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, world_position) as i32,
            );
            let loc = gl
                .get_attrib_location(program, "vert_scale")
                .ok_or_else(|| "No vert_scale attribute".to_string())?;
            gl.enable_vertex_attrib_array(loc);
            gl.vertex_attrib_pointer_f32(
                loc,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, scale) as i32,
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

            let model_matrix_location = gl
                .get_uniform_location(program, "model")
                .ok_or_else(|| "No model matrix attribute".to_string())?;
            let lorentz_matrix_location = gl
                .get_uniform_location(program, "lorentz")
                .ok_or_else(|| "No lorentz matrix attribute".to_string())?;
            let view_perspective_location = gl
                .get_uniform_location(program, "view_perspective")
                .ok_or_else(|| "No view_perspective matrix attribute".to_string())?;

            Ok(Self {
                gl,
                vbo,
                ebo,
                model_matrix_location,
                lorentz_matrix_location,
                view_perspective_location,
                triangle_count: indices.len(),
            })
        }
    }

    pub fn draw(
        &self,
        vertices: &[Vertex],
        model: Matrix,
        lorentz: Matrix,
        view_perspective: Matrix,
    ) -> Result<(), String> {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::DYNAMIC_DRAW,
            );
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));

            self.gl.uniform_matrix_4_f32_slice(
                Some(&self.model_matrix_location),
                false,
                &model.open_gl(),
            );
            self.gl.uniform_matrix_4_f32_slice(
                Some(&self.lorentz_matrix_location),
                false,
                &lorentz.open_gl(),
            );
            self.gl.uniform_matrix_4_f32_slice(
                Some(&self.view_perspective_location),
                false,
                &view_perspective.open_gl(),
            );
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
    vertices: Vec<Vertex>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
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
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let num = 16;
        let d = 8.0;
        for x in 0..num {
            for y in 0..num {
                for z in 0..num {
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
                            local_position: v,
                            world_position: [x as f32 * d, y as f32 * d, z as f32 * d],
                            scale: [3.0, 1.0, 0.5],
                            color,
                        });
                    }
                }
            }
        }
        Ok(App {
            backend: Backend::new(context, &indices).map_err(wasm_error)?,
            key_manager: KeyManager::new(),
            last_tick: None,
            player: Player::new(),
            vertices,
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
        let rot_matrix = Matrix::from(self.player.quaternion);
        let transition_matrix = Matrix::translation(-self.player.phase_space.position.spatial());
        let lorentz_matrix = Matrix::lorentz(self.player.phase_space.velocity);
        self.backend
            .draw(
                &self.vertices,
                transition_matrix,
                lorentz_matrix,
                projection_matrix * rot_matrix,
            )
            .map_err(wasm_error)
    }
}

pub struct Player {
    phase_space: PhaseSpace,
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
            phase_space: PhaseSpace::new(
                Vector3::zero(),
                Vector4::from_tv(0.0, vec3(50.0, 50.0, 40.0)),
            ),
            quaternion: Quaternion::from_axis(Deg(130.0), vec3(-1.0, 1.0, 0.0)),
        }
    }

    pub fn tick(&mut self, dt: f64, key: &KeyManager) {
        let a =
            self.get_user_input_acceleration(key) * 0.5 + self.get_viscous_acceleration() * 0.05;
        self.phase_space.tick(dt, a);
        self.quaternion *= self.get_rotation_velocity(dt, key);
    }

    pub fn get_user_input_acceleration(&self, key: &KeyManager) -> Vector3 {
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
        d.safe_normalized()
    }

    pub fn get_viscous_acceleration(&self) -> Vector3 {
        -self.phase_space.velocity
    }

    pub fn get_rotation_velocity(&self, dt: f64, key: &KeyManager) -> Quaternion {
        let mut right = 0.0;
        if key.is_pressed("arrowright") {
            right += 1.0;
        }
        if key.is_pressed("arrowleft") {
            right -= 1.0;
        }
        let mut up = 0.0;
        if key.is_pressed("arrowup") {
            up += 1.0;
        }
        if key.is_pressed("arrowdown") {
            up -= 1.0;
        }
        let mut role = 0.0;
        if key.is_pressed("e") {
            role += 1.0;
        }
        if key.is_pressed("q") {
            role -= 1.0;
        }
        if (right, up, role) == (0.0, 0.0, 0.0) {
            Quaternion::one()
        } else {
            let axis = self.quaternion.up() * right - self.quaternion.right() * up
                + self.quaternion.front() * role;
            Quaternion::from_axis(Rad(dt), axis)
        }
    }
}
