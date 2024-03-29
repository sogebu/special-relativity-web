use crate::{AddFace, BuildData, Data, Face, VertexPositionCalcNormal};

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

    pub fn size(mut self, size: f32) -> CubeOption {
        self.size = size;
        self
    }

    pub fn center(mut self, center: [f32; 3]) -> CubeOption {
        self.center = center;
        self
    }
}

impl BuildData for CubeOption {
    fn build<V>(&self) -> Data<V>
    where
        V: From<VertexPositionCalcNormal>,
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

    #[test]
    fn no_normal() {
        let cube = CubeOption::new().build_no_normal();
        assert_eq!(cube.vertices.len(), 8);
        assert_eq!(cube.triangles.len(), 12);
    }

    #[test]
    fn face_normal() {
        let cube = CubeOption::new().build_sharp();
        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.vertices[0].normal, [0.0, 0.0, 1.0]);
        assert_eq!(cube.triangles.len(), 12);
    }

    #[test]
    fn vert_normal() {
        let cube = CubeOption::new().build_smooth();
        assert_eq!(cube.vertices.len(), 8);
        for v in cube.vertices.iter() {
            for x in v.normal {
                assert_eq!(x.abs(), (1f32 / 3.0).sqrt());
            }
        }
        assert_eq!(cube.triangles.len(), 12);
    }
}
