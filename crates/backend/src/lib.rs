mod shader_lighting;
mod shader_lorentz;
mod shader_simple;
mod shape;

use glow::HasContext;

pub use self::{shader_lighting::*, shader_lorentz::*, shader_simple::*, shape::*};
pub struct Backend<C> {
    gl: C,
}

pub trait Shader<C> {
    type SharedData;
    type LocalData;

    fn bind_shared_data(&self, backend: &Backend<C>, data: &Self::SharedData);

    fn draw(
        &self,
        backend: &Backend<C>,
        shared_data: &Self::SharedData,
        local_data: &Self::LocalData,
    );
}

impl<C: HasContext> Backend<C> {
    pub fn new(gl: C) -> Result<Self, String> {
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

#[derive(Debug)]
struct VertexAttrib {
    index: u32,
    size: i32,
    stride: i32,
    offset: i32,
}

impl VertexAttrib {
    fn new<C: HasContext>(
        gl: &C,
        program: C::Program,
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

    fn bind<C: HasContext>(&self, gl: &C) {
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

fn make_program<C: HasContext>(
    gl: &C,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
) -> Result<C::Program, String> {
    fn make_shader<C: HasContext>(
        gl: &C,
        program: C::Program,
        sharder_type: u32,
        source: &str,
    ) -> Result<C::Shader, String> {
        unsafe {
            let shader = gl.create_shader(sharder_type)?;
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                return Err(gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            Ok(shader)
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

fn make_buffer<C: HasContext>(
    gl: &C,
    program: C::Program,
) -> Result<(C::Buffer, C::Buffer), String> {
    unsafe {
        gl.use_program(Some(program));
        let vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        let ebo = gl.create_buffer()?;
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        Ok((vbo, ebo))
    }
}

fn get_uniform_location<C: HasContext>(
    gl: &C,
    program: C::Program,
    name: &str,
) -> Result<C::UniformLocation, String> {
    unsafe {
        gl.get_uniform_location(program, name)
            .ok_or_else(|| format!("No '{}' uniform attribute", name))
    }
}
