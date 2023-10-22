use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext, WebBufferKey, WebProgramKey, WebShaderKey};
use memoffset::offset_of;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use rmath::Matrix;

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
    pub local_position: [f32; 3],
    pub world_position: [f32; 3],
    pub scale: [f32; 3],
    pub color: RGBA,
}

impl Backend {
    pub fn new(webgl2: WebGl2RenderingContext, indices: &[[u32; 3]]) -> Result<Self, String> {
        let gl = Context::from_webgl2_context(webgl2);
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.9, 0.9, 0.9, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            let program = make_program(
                &gl,
                include_str!("assets/vertex_shader.glsl"),
                include_str!("assets/fragment_shader.glsl"),
            )?;

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

    /// get (width, height)
    pub fn get_viewport_size(&self) -> (i32, i32) {
        let mut buf = [0; 4];
        unsafe {
            self.gl.get_parameter_i32_slice(glow::VIEWPORT, &mut buf);
            (buf[2] - buf[0], buf[3] - buf[1])
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
            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));

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

fn make_program(
    gl: &Context,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<WebProgramKey, String> {
    fn make_shader(
        gl: &Context,
        program: WebProgramKey,
        sharder_type: u32,
        source: &str,
    ) -> Result<WebShaderKey, String> {
        unsafe {
            let sharder = gl.create_shader(sharder_type)?;
            gl.shader_source(sharder, source);
            gl.compile_shader(sharder);
            if !gl.get_shader_compile_status(sharder) {
                return Err(gl.get_shader_info_log(sharder));
            }
            gl.attach_shader(program, sharder);
            Ok(sharder)
        }
    }
    unsafe {
        let program = gl.create_program()?;
        let vs = make_shader(gl, program, glow::VERTEX_SHADER, vertex_shader_source)?;
        let fs = make_shader(gl, program, glow::FRAGMENT_SHADER, fragment_shader_source)?;
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            return Err(gl.get_program_info_log(program));
        }
        gl.detach_shader(program, vs);
        gl.delete_shader(vs);
        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        gl.use_program(Some(program));
        Ok(program)
    }
}
