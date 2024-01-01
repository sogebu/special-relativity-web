use crate::backend::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, VertexAttrib,
    VertexPositionNormal,
};
use color::RGBA;
use glow::{HasContext, UniformLocation, WebBufferKey, WebProgramKey};
use memoffset::offset_of;
use rmath::Matrix;

pub struct LightingShader {
    program: WebProgramKey,
    vbo: WebBufferKey,
    ebo: WebBufferKey,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: UniformLocation,
    model_view_projection_location: UniformLocation,
    normal_location: UniformLocation,
}

pub struct LightingLocalData {
    pub color: RGBA,
    pub model_view_projection: Matrix,
    pub normal: Matrix,
}

impl LightingShader {
    pub fn new(backend: &Backend) -> Result<LightingShader, String> {
        let gl = &backend.gl;
        let program = make_program(
            gl,
            include_str!("glsl/lighting_vertex_shader.glsl"),
            include_str!("glsl/lighting_fragment_shader.glsl"),
        )?;
        let (vbo, ebo) = make_buffer(gl, program)?;

        let mut vertex_attrib = Vec::new();
        vertex_attrib.push(VertexAttrib::new(
            gl,
            program,
            "vert_local_position",
            3,
            std::mem::size_of::<VertexPositionNormal>(),
            offset_of!(VertexPositionNormal, position),
        )?);
        vertex_attrib.push(VertexAttrib::new(
            gl,
            program,
            "vert_normal",
            3,
            std::mem::size_of::<VertexPositionNormal>(),
            offset_of!(VertexPositionNormal, normal),
        )?);

        Ok(LightingShader {
            program,
            vbo,
            ebo,
            color_location: get_uniform_location(gl, program, "uniform_color")?,
            model_view_projection_location: get_uniform_location(
                gl,
                program,
                "model_view_projection_matrix",
            )?,
            normal_location: get_uniform_location(gl, program, "normal_matrix")?,
            vertex_attrib,
        })
    }
}

impl Shader for LightingShader {
    type SharedData = Shape<VertexPositionNormal>;
    type LocalData = LightingLocalData;

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
            gl.uniform_matrix_3_f32_slice(
                Some(&self.normal_location),
                false,
                &local_data.normal.open_gl_mat3(),
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
