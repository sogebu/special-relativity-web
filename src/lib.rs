use glow::{Buffer, Context, HasContext};
use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use color::RGBA;
use rmath::Vector3;

fn glow_error(s: String) -> anyhow::Error {
    anyhow::anyhow!("Glow Error: {}", s)
}

fn wasm_error(e: anyhow::Error) -> JsValue {
    e.to_string().into()
}

fn log(s: String) {
    console::log_1(&s.into());
}

pub struct Backend {
    gl: Context,
    position_location: u32,
    color_location: u32,
    vbo: Buffer,
    cbo: Buffer,
}

impl Backend {
    pub fn new(webgl2: WebGl2RenderingContext) -> anyhow::Result<Self> {
        let gl = Context::from_webgl2_context(webgl2);
        unsafe {
            gl.clear_color(0.9, 0.9, 0.9, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let program = gl.create_program().map_err(glow_error)?;
            let fragment_shader_source = include_str!("fragment_shader.glsl");
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).map_err(glow_error)?;
            gl.shader_source(vertex_shader, include_str!("vertex_shader.glsl"));
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_shader_info_log(vertex_shader)
                ));
            }
            gl.attach_shader(program, vertex_shader);
            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(glow_error)?;
            gl.shader_source(fragment_shader, fragment_shader_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_shader_info_log(fragment_shader)
                ));
            }
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_program_info_log(program)
                ));
            }
            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);
            gl.use_program(Some(program));

            let position_location = gl
                .get_attrib_location(program, "vert_position")
                .ok_or_else(|| anyhow::anyhow!("No vert_position attribute"))?;
            let color_location = gl
                .get_attrib_location(program, "vert_color")
                .ok_or_else(|| anyhow::anyhow!("No vert_color attribute"))?;
            let vbo = gl.create_buffer().map_err(glow_error)?;
            let cbo = gl.create_buffer().map_err(glow_error)?;

            Ok(Self {
                gl,
                position_location,
                color_location,
                vbo,
                cbo,
            })
        }
    }

    pub fn draw(&self, d: Vector3) -> anyhow::Result<()> {
        let vertices: &[Vector3] = &[
            d + Vector3::new(-0.5, 0.5, 0.0),
            d + Vector3::new(-0.5, -0.5, 0.0),
            d + Vector3::new(0.5, 0.5, 0.0),
            d + Vector3::new(-0.5, -0.5, 0.0),
            d + Vector3::new(0.5, -0.5, 0.0),
            d + Vector3::new(0.5, 0.5, 0.0),
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
    last_tick: Option<f64>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        Ok(App {
            backend: Backend::new(context).map_err(wasm_error)?,
            last_tick: None,
        })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        let last_tick = self.last_tick.replace(timestamp);
        let dt = timestamp - last_tick.unwrap_or(timestamp);
        log(format!("{}", dt));

        let x = (timestamp / 10000.0) as f32;
        self.backend
            .draw(Vector3::new(x, x, 1.0))
            .map_err(wasm_error)
    }
}
