use glow::{HasContext, UniformLocation, WebBufferKey, WebProgramKey};
use memoffset::offset_of;

use super::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, Vertex, VertexAttrib,
};
use color::RGBA;
use rmath::Matrix;

pub struct LorentzShader {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: UniformLocation,
    model_matrix_location: UniformLocation,
    lorentz_matrix_location: UniformLocation,
    view_projection_location: UniformLocation,
}

pub struct LorentzLocalData {
    pub color: RGBA,
    pub model: Matrix,
    pub lorentz: Matrix,
    pub view_projection: Matrix,
}

impl LorentzShader {
    pub fn new(backend: &Backend) -> Result<LorentzShader, String> {
        let gl = &backend.gl;
        let program = make_program(
            gl,
            include_str!("glsl/lorentz_vertex_shader.glsl"),
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

        Ok(LorentzShader {
            program,
            vbo,
            ebo,
            color_location: get_uniform_location(gl, program, "uniform_color")?,
            model_matrix_location: get_uniform_location(gl, program, "model")?,
            lorentz_matrix_location: get_uniform_location(gl, program, "lorentz")?,
            view_projection_location: get_uniform_location(gl, program, "view_projection")?,
            vertex_attrib,
        })
    }
}

impl Shader for LorentzShader {
    type SharedData = Shape<Vertex>;
    type LocalData = LorentzLocalData;

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
                Some(&self.view_projection_location),
                false,
                &local_data.view_projection.open_gl(),
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
