use glow::{Buffer, Context, HasContext};
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use key::KeyManager;
use rmath::{Matrix, Vector3};

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
    position_location: u32,
    color_location: u32,
    matrix_location: WebGlUniformLocation,
    vbo: Buffer,
    cbo: Buffer,
}

impl Backend {
    pub fn new(webgl2: WebGl2RenderingContext) -> Result<Self, String> {
        let gl = Context::from_webgl2_context(webgl2);
        unsafe {
            gl.clear_color(0.9, 0.9, 0.9, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

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

            let position_location = gl
                .get_attrib_location(program, "vert_position")
                .ok_or_else(|| "No vert_position attribute".to_string())?;
            let color_location = gl
                .get_attrib_location(program, "vert_color")
                .ok_or_else(|| "No vert_color attribute".to_string())?;
            let matrix_location = gl
                .get_uniform_location(program, "matrix")
                .ok_or_else(|| "No matrix attribute".to_string())?;

            let vbo = gl.create_buffer()?;
            let cbo = gl.create_buffer()?;

            Ok(Self {
                gl,
                position_location,
                color_location,
                matrix_location,
                vbo,
                cbo,
            })
        }
    }

    pub fn draw(&self, mat: Matrix) -> Result<(), String> {
        let vertices: &[Vector3] = &[
            Vector3::new(-0.5, 0.5, 0.0),
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(0.5, 0.5, 0.0),
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(0.5, -0.5, 0.0),
            Vector3::new(0.5, 0.5, 0.0),
        ];
        let colors = &[
            RGBA::red(),
            RGBA::lime(),
            RGBA::blue(),
            RGBA::lime(),
            RGBA::black(),
            RGBA::blue(),
        ];

        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STREAM_DRAW,
            );
            self.gl.enable_vertex_attrib_array(self.position_location);
            self.gl
                .vertex_attrib_pointer_f32(self.position_location, 3, glow::FLOAT, false, 0, 0);

            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.cbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(colors),
                glow::STREAM_DRAW,
            );

            self.gl.enable_vertex_attrib_array(self.color_location);
            self.gl
                .vertex_attrib_pointer_f32(self.color_location, 4, glow::FLOAT, false, 0, 0);

            self.gl
                .uniform_matrix_4_f32_slice(Some(&self.matrix_location), false, &mat.array());

            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
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
    base: Vector3,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        Ok(App {
            backend: Backend::new(context).map_err(wasm_error)?,
            key_manager: KeyManager::new(),
            last_tick: None,
            base: Vector3::new(0.0, 0.0, 0.0),
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
        self.base += Player.get_velocity(&self.key_manager) * dt as f32;
        self.backend
            .draw(Matrix::translation(self.base))
            .map_err(wasm_error)
    }
}

pub struct Player;

impl Player {
    pub fn get_velocity(&self, key: &KeyManager) -> Vector3 {
        let mut up = 0.0;
        let mut right = 0.0;
        if key.is_pressed("w") {
            up += 1.0;
        }
        if key.is_pressed("s") {
            up -= 1.0;
        }
        if key.is_pressed("d") {
            right += 1.0;
        }
        if key.is_pressed("a") {
            right -= 1.0;
        }
        let mut d = Vector3::new(right, up, 0.0);
        if d.magnitude2() > 0.0 {
            d /= d.magnitude();
        };
        d
    }
}
