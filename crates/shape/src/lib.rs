mod arrow;
mod cube;
mod icosahedron;

pub use crate::{arrow::ArrowOption, cube::CubeOption, icosahedron::IcosahedronOption};
use rmath::Vector3;

#[derive(Debug, Clone)]
pub struct Data<V> {
    pub vertices: Vec<V>,
    pub triangles: Vec<[u32; 3]>,
}

impl<V> Data<V> {
    pub fn with_capacity(v: usize, t: usize) -> Data<V> {
        Data {
            vertices: Vec::with_capacity(v),
            triangles: Vec::with_capacity(t),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

pub trait AddFace {
    fn add_face(&mut self, face: &Face);
}

pub struct Face {
    pub vertices: Vec<[f32; 3]>,
}

impl AddFace for Data<[f32; 3]> {
    fn add_face(&mut self, face: &Face) {
        fn push_vertex(data: &mut Data<[f32; 3]>, v: [f32; 3]) -> u32 {
            for (i, &p) in data.vertices.iter().enumerate() {
                if v == p {
                    return i as u32;
                }
            }
            data.vertices.push(v);
            data.vertices.len() as u32 - 1
        }

        let i0 = push_vertex(self, face.vertices[0]);
        for i in 1..face.vertices.len() - 1 {
            let i1 = push_vertex(self, face.vertices[i]);
            let i2 = push_vertex(self, face.vertices[i + 1]);
            self.triangles.push([i0, i1, i2]);
        }
    }
}

impl AddFace for Data<Vertex> {
    fn add_face(&mut self, face: &Face) {
        let base = self.vertices.len() as u32;
        for i in 1..face.vertices.len() as u32 - 1 {
            self.triangles.push([base, base + i, base + i + 1]);
        }
        let normal = face.normal();
        self.vertices.extend(
            face.vertices
                .iter()
                .map(|&position| Vertex { position, normal }),
        )
    }
}

impl Face {
    pub fn normal(&self) -> [f32; 3] {
        let i = Vector3::from(self.vertices[0]);
        let j = Vector3::from(self.vertices[1]);
        let k = Vector3::from(self.vertices[2]);
        let a = j - i;
        let b = k - j;
        a.cross(b).normalized().into()
    }
}

impl Data<[f32; 3]> {
    #[cfg(feature = "obj")]
    pub fn write_as_obj<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for [x, y, z] in self.vertices.iter() {
            write!(w, "v {} {} {}\n", x, y, z)?;
        }
        for &[x, y, z] in self.triangles.iter() {
            write!(w, "f {} {} {}\n", x + 1, y + 1, z + 1)?;
        }
        Ok(())
    }
}

impl Data<Vertex> {
    #[cfg(feature = "obj")]
    pub fn write_as_obj<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for v in self.vertices.iter() {
            let [x, y, z] = v.position;
            writeln!(w, "v {x} {y} {z}")?;
            let [x, y, z] = v.normal;
            writeln!(w, "vn {x} {y} {z}")?;
        }
        for &[x, y, z] in self.triangles.iter() {
            let x = x + 1;
            let y = y + 1;
            let z = z + 1;
            writeln!(w, "f {x}//{x} {y}//{y} {z}//{z}")?;
        }
        Ok(())
    }
}
