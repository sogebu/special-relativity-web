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
    use crate::Vertex;

    #[test]
    fn no_normal() {
        let cube = CubeOption::new().build::<[f32; 3]>();
        assert_eq!(cube.vertices.len(), 8);
        assert_eq!(cube.triangles.len(), 12);
    }

    #[test]
    fn face_normal() {
        let cube = CubeOption::new().build::<Vertex>();
        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.triangles.len(), 12);
    }
}
