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

pub trait BuildData {
    fn build<V>(&self) -> Data<V>
    where
        V: From<VertexPositionCalcNormal>,
        Data<V>: AddFace;

    fn build_sharp(&self) -> Data<VertexPositionNormal> {
        self.build::<VertexPositionNormal>()
    }

    fn build_smooth(&self) -> Data<VertexPositionNormal> {
        self.build::<VertexPositionCalcNormal>()
            .vertex_converted::<VertexPositionNormal>()
    }

    fn build_no_normal(&self) -> Data<VertexPosition> {
        self.build::<VertexPosition>()
    }
}

impl<V> Data<V> {
    pub fn with_capacity(v: usize, t: usize) -> Data<V> {
        Data {
            vertices: Vec::with_capacity(v),
            triangles: Vec::with_capacity(t),
        }
    }

    pub fn append(&mut self, other: Data<V>) {
        let n = self.vertices.len() as u32;
        self.vertices.extend(other.vertices);
        self.triangles.extend(
            other
                .triangles
                .iter()
                .map(|&[i, j, k]| [n + i, n + j, n + k]),
        );
    }
}

impl<V: Copy> Data<V> {
    pub fn vertex_converted<W: From<V>>(&self) -> Data<W> {
        Data {
            vertices: self.vertices.iter().map(|&v| v.into()).collect(),
            triangles: self.triangles.clone(),
        }
    }
}

impl Data<VertexPosition> {
    pub fn dedup(&self) -> Data<VertexPosition> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        'OUT: for v in self.vertices.iter() {
            for (i, p) in vertices.iter().enumerate() {
                if v == p {
                    indices.push(i);
                    continue 'OUT;
                }
            }
            indices.push(vertices.len());
            vertices.push(*v);
        }
        Data {
            vertices,
            triangles: self
                .triangles
                .iter()
                .map(|&[i, j, k]| {
                    [
                        indices[i as usize] as u32,
                        indices[j as usize] as u32,
                        indices[k as usize] as u32,
                    ]
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPosition {
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPositionNormal {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexPositionCalcNormal {
    pub position: [f32; 3],
    pub normal: Vector3,
}

pub trait AddFace {
    fn add_face(&mut self, face: &Face);
}

pub struct Face {
    pub vertices: Vec<[f32; 3]>,
}

impl AddFace for Data<VertexPosition> {
    fn add_face(&mut self, face: &Face) {
        fn push_vertex(data: &mut Data<VertexPosition>, p: [f32; 3]) -> u32 {
            for (i, &v) in data.vertices.iter().enumerate() {
                if p == v.position {
                    return i as u32;
                }
            }
            data.vertices.push(VertexPosition { position: p });
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

impl AddFace for Data<VertexPositionNormal> {
    fn add_face(&mut self, face: &Face) {
        let base = self.vertices.len() as u32;
        for i in 1..face.vertices.len() as u32 - 1 {
            self.triangles.push([base, base + i, base + i + 1]);
        }
        let normal = face.normal().into();
        self.vertices.extend(
            face.vertices
                .iter()
                .map(|&position| VertexPositionNormal { position, normal }),
        )
    }
}

impl AddFace for Data<VertexPositionCalcNormal> {
    fn add_face(&mut self, face: &Face) {
        let normal = face.normal();
        let mut indices = Vec::with_capacity(face.vertices.len());
        'OUT: for &v in face.vertices.iter() {
            for (i, p) in self.vertices.iter_mut().enumerate() {
                if p.position == v {
                    p.normal += normal;
                    indices.push(i);
                    continue 'OUT;
                }
            }
            indices.push(self.vertices.len());
            self.vertices.push(VertexPositionCalcNormal {
                position: v,
                normal,
            });
        }

        let i0 = indices[0] as u32;
        for i in 1..face.vertices.len() - 1 {
            let i1 = indices[i] as u32;
            let i2 = indices[i + 1] as u32;
            self.triangles.push([i0, i1, i2]);
        }
    }
}

impl Face {
    pub fn normal(&self) -> Vector3 {
        let i = Vector3::from(self.vertices[0]);
        let j = Vector3::from(self.vertices[1]);
        let k = Vector3::from(self.vertices[2]);
        let a = j - i;
        let b = k - j;
        a.cross(b).normalized()
    }
}

impl From<VertexPositionCalcNormal> for VertexPositionNormal {
    fn from(v: VertexPositionCalcNormal) -> Self {
        VertexPositionNormal {
            position: v.position,
            normal: v.normal.normalized().into(),
        }
    }
}

impl From<VertexPositionNormal> for VertexPosition {
    fn from(v: VertexPositionNormal) -> Self {
        VertexPosition {
            position: v.position,
        }
    }
}

impl From<VertexPositionCalcNormal> for VertexPosition {
    fn from(v: VertexPositionCalcNormal) -> Self {
        VertexPosition {
            position: v.position,
        }
    }
}

impl Data<VertexPosition> {
    #[cfg(feature = "obj")]
    pub fn write_as_obj<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for [x, y, z] in self.vertices.iter().map(|v| v.position) {
            write!(w, "v {} {} {}\n", x, y, z)?;
        }
        for &[x, y, z] in self.triangles.iter() {
            write!(w, "f {} {} {}\n", x + 1, y + 1, z + 1)?;
        }
        Ok(())
    }
}

impl Data<VertexPositionNormal> {
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
