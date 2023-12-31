use crate::backend::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, Vertex, VertexAttrib,
};
use color::RGBA;
use glow::{HasContext, UniformLocation, WebBufferKey, WebProgramKey};
use memoffset::offset_of;
use rmath::Matrix;

pub struct JustShader {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: UniformLocation,
    model_view_perspective_location: UniformLocation,
}

pub struct JustLocalData {
    pub color: RGBA,
    pub model_view_perspective: Matrix,
}

impl JustShader {
    pub fn new(backend: &Backend) -> Result<JustShader, String> {
        let gl = &backend.gl;
        let program = make_program(
            gl,
            include_str!("glsl/just_vertex_shader.glsl"),
            include_str!("glsl/fragment_shader.glsl"),
        )?;
        let (vbo, ebo) = make_buffer(gl, program)?;

        let mut vertex_attrib = Vec::new();
        vertex_attrib.push(VertexAttrib::new(
            gl,
            program,
            "vert_local_position",
            3,
            std::mem::size_of::<Vertex>(),
            offset_of!(Vertex, position),
        )?);

        Ok(JustShader {
            program,
            vbo,
            ebo,
            color_location: get_uniform_location(gl, program, "uniform_color")?,
            model_view_perspective_location: get_uniform_location(
                gl,
                program,
                "model_view_perspective",
            )?,
            vertex_attrib,
        })
    }
}

impl Shader for JustShader {
    type SharedData = Shape<Vertex>;
    type LocalData = JustLocalData;

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
            gl.uniform_4_f32(
                Some(&self.color_location),
                local_data.color.r,
                local_data.color.g,
                local_data.color.b,
                local_data.color.a,
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.model_view_perspective_location),
                false,
                &local_data.model_view_perspective.open_gl(),
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
