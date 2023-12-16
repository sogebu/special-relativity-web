mod cube;

pub use crate::cube::CubeOption;

#[derive(Debug, Clone)]
pub struct Data {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<[u32; 3]>,
}

impl Data {
    #[cfg(feature = "obj")]
    pub fn write_as_obj<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for [x, y, z] in self.vertices.iter() {
            write!(w, "v {} {} {}\n", x, y, z)?
        }
        for &[x, y, z] in self.indices.iter() {
            write!(w, "f {}// {}// {}//\n", x + 1, y + 1, z + 1)?
        }
        Ok(())
    }
}
