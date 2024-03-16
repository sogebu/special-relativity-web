use crate::{
    get_uniform_location, make_buffer, make_program, Backend, Shader, Shape, VertexAttrib,
    VertexPosition,
};
use color::RGBA;
use glow::HasContext;
use memoffset::offset_of;
use rmath::Matrix;

pub struct SimpleShader<C: HasContext> {
    program: C::Program,
    vbo: C::Buffer,
    ebo: C::Buffer,
    vertex_attrib: Vec<VertexAttrib>,
    color_location: C::UniformLocation,
    model_view_projection_location: C::UniformLocation,
}

pub struct SimpleLocalData {
    pub color: RGBA,
    pub model_view_projection: Matrix,
}

impl<C: HasContext> SimpleShader<C> {
    pub fn new(backend: &Backend<C>) -> Result<SimpleShader<C>, String> {
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

impl<C: HasContext> Shader<C> for SimpleShader<C> {
    type SharedData = Shape<VertexPosition>;
    type LocalData = SimpleLocalData;

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
            backend.gl.draw_elements(
                glow::TRIANGLES,
                shared_data.elements_count(),
                glow::UNSIGNED_INT,
                0,
            );
        }
    }
}
