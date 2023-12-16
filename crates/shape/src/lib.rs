mod cube;

pub use crate::cube::CubeOption;

#[derive(Debug, Clone)]
pub struct Data {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<[u32; 3]>,
}
