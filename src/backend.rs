use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext, UniformLocation, WebBufferKey, WebProgramKey, WebShaderKey};
use memoffset::offset_of;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use color::RGBA;
use rmath::Matrix;

pub struct Backend {
    gl: Context,
}

pub trait Shader {
    type SharedData;
    type LocalData;

    fn bind_shared_data(&self, backend: &Backend, data: &Self::SharedData);

    fn draw(&self, backend: &Backend, shared_data: &Self::SharedData, local_data: &Self::LocalData);
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

    /// get (width, height)
    pub fn get_viewport_size(&self) -> (i32, i32) {
        let mut buf = [0; 4];
        unsafe {
            self.gl.get_parameter_i32_slice(glow::VIEWPORT, &mut buf);
            (buf[2] - buf[0], buf[3] - buf[1])
        }
    }

    pub fn clear(&self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn flush(&self) {
        unsafe {
            self.gl.flush();
        }
    }
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Vertex {
    pub local_position: [f32; 3],
}

pub struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<[u32; 3]>,
}

impl From<shape::Data> for Shape {
    fn from(value: shape::Data) -> Self {
        let vertices = value
            .vertices
            .iter()
            .map(|&v| Vertex { local_position: v })
            .collect();
        Shape {
            vertices,
            indices: value.indices,
        }
    }
}

pub struct LorentzShader {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: UniformLocation,
    model_matrix_location: UniformLocation,
    lorentz_matrix_location: UniformLocation,
    view_perspective_location: UniformLocation,
}

pub struct LorentzLocalData {
    pub color: RGBA,
    pub model: Matrix,
    pub lorentz: Matrix,
    pub view_perspective: Matrix,
}

impl LorentzShader {
    pub fn new(backend: &Backend) -> Result<LorentzShader, String> {
        let gl = &backend.gl;
        unsafe {
            let program = make_program(
                gl,
                include_str!("assets/vertex_shader.glsl"),
                include_str!("assets/fragment_shader.glsl"),
            )?;
            gl.use_program(Some(program));
            let vbo = gl.create_buffer()?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

            let mut vertex_attrib = Vec::new();
            vertex_attrib.push(VertexAttrib::new(
                gl,
                program,
                "vert_local_position",
                3,
                std::mem::size_of::<Vertex>(),
                offset_of!(Vertex, local_position),
            )?);

            Ok(LorentzShader {
                program,
                vbo,
                ebo,
                color_location: get_uniform_location(gl, program, "uniform_color")?,
                model_matrix_location: get_uniform_location(gl, program, "model")?,
                lorentz_matrix_location: get_uniform_location(gl, program, "lorentz")?,
                view_perspective_location: get_uniform_location(gl, program, "view_perspective")?,
                vertex_attrib,
            })
        }
    }
}

impl Shader for LorentzShader {
    type SharedData = Shape;
    type LocalData = LorentzLocalData;

    fn bind_shared_data(&self, backend: &Backend, data: &Self::SharedData) {
        let gl = &backend.gl;
        unsafe {
            gl.use_program(Some(self.program));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&data.vertices),
                glow::STATIC_READ,
            );
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&data.indices),
                glow::STATIC_READ,
            );

            for va in self.vertex_attrib.iter() {
                va.bind(gl);
            }
        }
    }

    fn draw(
        &self,
        backend: &Backend,
        shared_data: &Self::SharedData,
        local_data: &Self::LocalData,
    ) {
        let gl = &backend.gl;
        unsafe {
            gl.uniform_4_f32(
                Some(&self.color_location),
                local_data.color.r,
                local_data.color.g,
                local_data.color.b,
                local_data.color.a,
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.model_matrix_location),
                false,
                &local_data.model.open_gl(),
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.lorentz_matrix_location),
                false,
                &local_data.lorentz.open_gl(),
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.view_perspective_location),
                false,
                &local_data.view_perspective.open_gl(),
            );
            backend.gl.draw_elements(
                glow::TRIANGLES,
                (shared_data.indices.len() * 3) as i32,
                glow::UNSIGNED_INT,
                0,
            );
        }
    }
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

fn get_uniform_location(
    gl: &Context,
    program: WebProgramKey,
    name: &str,
) -> Result<WebGlUniformLocation, String> {
    unsafe {
        gl.get_uniform_location(program, name)
            .ok_or_else(|| format!("No '{}' uniform attribute", name))
    }
}
