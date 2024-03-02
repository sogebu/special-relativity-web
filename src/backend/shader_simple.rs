use crate::backend::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, VertexAttrib,
    VertexPosition,
};
use color::RGBA;
use glow::{HasContext, UniformLocation, WebBufferKey, WebProgramKey};
use memoffset::offset_of;
use rmath::Matrix;

pub struct SimpleShader {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: UniformLocation,
    model_view_projection_location: UniformLocation,
}

pub struct SimpleLocalData {
    pub color: RGBA,
    pub model_view_projection: Matrix,
}

impl SimpleShader {
    pub fn new(backend: &Backend) -> Result<SimpleShader, String> {
        let gl = &backend.gl;
        let program = make_program(
            gl,
            include_str!("glsl/simple_vertex_shader.glsl"),
            include_str!("glsl/fragment_shader.glsl"),
        )?;
        let (vbo, ebo) = make_buffer(gl, program)?;

        let mut vertex_attrib = Vec::new();
        vertex_attrib.push(VertexAttrib::new(
            gl,
            program,
            "vert_local_position",
            3,
            std::mem::size_of::<VertexPosition>(),
            offset_of!(VertexPosition, position),
        )?);

        Ok(SimpleShader {
            program,
            vbo,
            ebo,
            color_location: get_uniform_location(gl, program, "uniform_color")?,
            model_view_projection_location: get_uniform_location(
                gl,
                program,
                "model_view_projection",
            )?,
            vertex_attrib,
        })
    }
}

impl Shader for SimpleShader {
    type SharedData = Shape<VertexPosition>;
    type LocalData = SimpleLocalData;

    fn bind_shared_data(&self, backend: &Backend, data: &Self::SharedData) {
        let gl = &backend.gl;
        data.bind(gl, self.program, self.vbo, self.ebo);
        for va in self.vertex_attrib.iter() {
            va.bind(gl);
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
            gl.uniform_4_f32_slice(Some(&self.color_location), &local_data.color.as_array());
            gl.uniform_matrix_4_f32_slice(
                Some(&self.model_view_projection_location),
                false,
                &local_data.model_view_projection.open_gl(),
            );
            backend.gl.draw_elements(
                glow::TRIANGLES,
                shared_data.elements_count(),
                glow::UNSIGNED_INT,
                0,
            );
        }
    }
}