use crate::{AddFace, Data, Face};

#[derive(Debug)]
pub struct CubeOption {
    size: f32,
    center: [f32; 3],
}

impl Default for CubeOption {
    fn default() -> Self {
        CubeOption {
            size: 1.0,
            center: [0.0; 3],
        }
    }
}

impl CubeOption {
    pub fn new() -> CubeOption {
        Default::default()
    }

    pub fn size(&mut self, size: f32) -> &mut CubeOption {
        self.size = size;
        self
    }

    pub fn center(&mut self, center: [f32; 3]) -> &mut CubeOption {
        self.center = center;
        self
    }

    pub fn build<V>(&self) -> Data<V>
    where
        Data<V>: AddFace,
    {
        let mut data = Data::<V>::with_capacity(8, 12);
        let [x, y, z] = self.center;
        let h = self.size / 2.0;
        data.add_face(&Face {
            vertices: vec![
                [x + h, y + h, z + h],
                [x - h, y + h, z + h],
                [x - h, y - h, z + h],
                [x + h, y - h, z + h],
            ],
        });
        data.add_face(&Face {
            vertices: vec![
                [x + h, y + h, z + h],
                [x + h, y + h, z - h],
                [x - h, y + h, z - h],
                [x - h, y + h, z + h],
            ],
        });
        data.add_face(&Face {
            vertices: vec![
                [x + h, y + h, z + h],
                [x + h, y - h, z + h],
                [x + h, y - h, z - h],
                [x + h, y + h, z - h],
            ],
        });
        data.add_face(&Face {
            vertices: vec![
                [x - h, y - h, z - h],
                [x - h, y - h, z + h],
                [x - h, y + h, z + h],
                [x - h, y + h, z - h],
            ],
        });
        data.add_face(&Face {
            vertices: vec![
                [x - h, y - h, z - h],
                [x - h, y + h, z - h],
                [x + h, y + h, z - h],
                [x + h, y - h, z - h],
            ],
        });
        data.add_face(&Face {
            vertices: vec![
                [x - h, y - h, z - h],
                [x + h, y - h, z - h],
                [x + h, y - h, z + h],
                [x - h, y - h, z + h],
            ],
        });
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VertexA, VertexB};

    #[test]
    fn no_normal() {
        let cube = CubeOption::new().build::<[f32; 3]>();
        assert_eq!(cube.vertices.len(), 8);
        assert_eq!(cube.triangles.len(), 12);
    }

    #[test]
    fn face_normal() {
        let cube = CubeOption::new().build::<VertexA>();
        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.vertices[0].normal, [0.0, 0.0, 1.0]);
        assert_eq!(cube.triangles.len(), 12);
    }

    #[test]
    fn vert_normal() {
        let cube = CubeOption::new()
            .build::<VertexB>()
            .vertex_converted::<VertexA>();
        assert_eq!(cube.vertices.len(), 8);
        for v in cube.vertices.iter() {
            for x in v.normal {
                assert_eq!(x.abs(), (1f32 / 3.0).sqrt());
            }
        }
        assert_eq!(cube.triangles.len(), 12);
    }
}
