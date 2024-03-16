use crate::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, VertexAttrib,
    VertexPositionNormal,
};
use color::RGBA;
use glow::HasContext;
use memoffset::offset_of;
use rmath::Matrix;

pub struct LightingShader<C: HasContext> {
    program: C::Program,
    vbo: C::Buffer,
    ebo: C::Buffer,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: C::UniformLocation,
    model_view_projection_location: C::UniformLocation,
    normal_location: C::UniformLocation,
}

pub struct LightingLocalData {
    pub color: RGBA,
    pub model_view_projection: Matrix,
    pub normal: Matrix,
}

impl<C: HasContext> LightingShader<C> {
    pub fn new(backend: &Backend<C>) -> Result<LightingShader<C>, String> {
        let gl = &backend.gl;
        let program = make_program(
            gl,
            include_str!("glsl/lighting_vertex_shader.glsl"),
            include_str!("glsl/fragment_shader.glsl"),
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

impl<C: HasContext> Shader<C> for LightingShader<C> {
    type SharedData = Shape<VertexPositionNormal>;
    type LocalData = LightingLocalData;

    fn bind_shared_data(&self, backend: &Backend<C>, data: &Self::SharedData) {
        let gl = &backend.gl;
        data.bind(gl, self.program, self.vbo, self.ebo);
        for va in self.vertex_attrib.iter() {
            va.bind(gl);
        }
    }

    fn draw(
        &self,
        backend: &Backend<C>,
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
