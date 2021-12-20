use color::RGBA;
use glow::{Buffer, Context, HasContext};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn glow_error(s: String) -> anyhow::Error {
    anyhow::anyhow!("Glow Error: {}", s)
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

    pub fn draw(&self) -> anyhow::Result<()> {
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            -0.5, 0.5,  0.0,
            -0.5, -0.5, 0.0,
            0.5,  0.5,  0.0,
            -0.5, -0.5, 0.0,
            0.5,  -0.5, 0.0,
            0.5,  0.5,  0.0,
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

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let canvas: HtmlCanvasElement = document()
        .get_element_by_id("canvas")
        .expect("No canvas")
        .dyn_into()
        .expect("No canvas");

    canvas.set_width(600);
    canvas.set_height(600);

    let webgl2: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .expect("This Platform is unsupported webgl2")
        .expect("No webgl2")
        .dyn_into()
        .expect("No webgl2");

    let backend = Backend::new(webgl2).unwrap();
    backend.draw().unwrap();
    Ok(())
}
