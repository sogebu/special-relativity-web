use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext, WebBufferKey, WebProgramKey, WebShaderKey};
use memoffset::offset_of;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use rmath::Matrix;

pub struct Backend {
    gl: Context,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vertex {
    pub local_position: [f32; 3],
    pub color: RGBA,
}

pub struct Entity {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertices: Vec<Vertex>,
    indices: Vec<[u32; 3]>,
    pub model_local_matrix: Matrix,
    model_matrix_location: WebGlUniformLocation,
    lorentz_matrix_location: WebGlUniformLocation,
    view_perspective_location: WebGlUniformLocation,
    triangle_count: usize,
    vertex_attrib: Vec<VertexAttrib>,
}

#[derive(Debug)]
struct VertexAttrib {
    index: u32,
    size: i32,
    stride: i32,
    offset: i32,
}

impl VertexAttrib {
    fn new(
        gl: &Context,
        program: WebProgramKey,
        name: &str,
        size: usize,
        stride: usize,
        offset: usize,
    ) -> Result<VertexAttrib, String> {
        let index = unsafe {
            gl.get_attrib_location(program, name)
                .ok_or_else(|| format!("No '{}' attribute", name))?
        };
        Ok(VertexAttrib {
            index,
            size: size as i32,
            stride: stride as i32,
            offset: offset as i32,
        })
    }

    fn bind(&self, gl: &Context) {
        unsafe {
            gl.vertex_attrib_pointer_f32(
                self.index,
                self.size,
                glow::FLOAT,
                false,
                self.stride,
                self.offset,
            );
            gl.enable_vertex_attrib_array(self.index);
        }
    }
}

impl Backend {
    pub fn new(webgl2: WebGl2RenderingContext) -> Result<Self, String> {
        let gl = Context::from_webgl2_context(webgl2);
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.9, 0.9, 0.9, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            Ok(Self { gl })
        }
    }

    pub fn new_entity(&self, vertices: &[Vertex], indices: &[[u32; 3]]) -> Result<Entity, String> {
        unsafe {
            let program = make_program(
                &self.gl,
                include_str!("assets/vertex_shader.glsl"),
                include_str!("assets/fragment_shader.glsl"),
            )?;
            self.gl.use_program(Some(program));

            let vbo = self.gl.create_buffer()?;
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = self.gl.create_buffer()?;
            self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

            let mut vertex_attrib = Vec::new();
            vertex_attrib.push(VertexAttrib::new(
                &self.gl,
                program,
                "vert_local_position",
                3,
                std::mem::size_of::<Vertex>(),
                offset_of!(Vertex, local_position),
            )?);
            vertex_attrib.push(VertexAttrib::new(
                &self.gl,
                program,
                "vert_color",
                3,
                std::mem::size_of::<Vertex>(),
                offset_of!(Vertex, color),
            )?);

            let model_matrix_location = self
                .gl
                .get_uniform_location(program, "model")
                .ok_or_else(|| "No model matrix attribute".to_string())?;
            let lorentz_matrix_location = self
                .gl
                .get_uniform_location(program, "lorentz")
                .ok_or_else(|| "No lorentz matrix attribute".to_string())?;
            let view_perspective_location = self
                .gl
                .get_uniform_location(program, "view_perspective")
                .ok_or_else(|| "No view_perspective matrix attribute".to_string())?;

            Ok(Entity {
                program,
                vbo,
                ebo,
                vertices: vertices.to_vec(),
                indices: indices.to_vec(),
                model_local_matrix: Matrix::ident(),
                model_matrix_location,
                lorentz_matrix_location,
                view_perspective_location,
                triangle_count: indices.len(),
                vertex_attrib,
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
        entities: &[Entity],
        model: Matrix,
        lorentz: Matrix,
        view_perspective: Matrix,
    ) -> Result<(), String> {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            for e in entities {
                e.draw(&self.gl, model, lorentz, view_perspective);
            }
            self.gl.flush();
        }
        Ok(())
    }
}

impl Entity {
    pub fn draw(&self, gl: &Context, model: Matrix, lorentz: Matrix, view_perspective: Matrix) {
        unsafe {
            gl.use_program(Some(self.program));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.vertices),
                glow::STATIC_READ,
            );
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.indices),
                glow::STATIC_READ,
            );

            for va in self.vertex_attrib.iter() {
                va.bind(gl);
            }

            let model = model * self.model_local_matrix;
            gl.uniform_matrix_4_f32_slice(
                Some(&self.model_matrix_location),
                false,
                &model.open_gl(),
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.lorentz_matrix_location),
                false,
                &lorentz.open_gl(),
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.view_perspective_location),
                false,
                &view_perspective.open_gl(),
            );
            gl.draw_elements(
                glow::TRIANGLES,
                (self.triangle_count * 3) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
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
        Ok(program)
    }
}
