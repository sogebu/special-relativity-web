use bytemuck::{NoUninit, Pod, Zeroable};
use glow::{Context, HasContext, WebBufferKey, WebProgramKey};

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub struct Shape<V> {
    vertices: Vec<V>,
    triangles: Vec<[u32; 3]>,
}

impl<V> Shape<V> {
    pub fn elements_count(&self) -> i32 {
        self.triangles.len() as i32 * 3
    }
}

impl<V> Shape<V>
where
    V: NoUninit,
{
    pub fn bind(&self, gl: &Context, program: WebProgramKey, vbo: WebBufferKey, ebo: WebBufferKey) {
        unsafe {
            gl.use_program(Some(program));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.vertices),
                glow::STATIC_READ,
            );
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.triangles),
                glow::STATIC_READ,
            );
        }
    }
}

impl From<shape::VertexPosition> for Vertex {
    fn from(v: shape::VertexPosition) -> Self {
        Vertex {
            position: v.position,
        }
    }
}

impl<V1, V2> From<shape::Data<V1>> for Shape<V2>
where
    V1: Into<V2>,
{
    fn from(data: shape::Data<V1>) -> Self {
        Shape {
            vertices: data.vertices.into_iter().map(|v| v.into()).collect(),
            triangles: data.triangles,
        }
    }
}
