use crate::Data;

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

    pub fn build(&self) -> Data {
        let [x, y, z] = self.center;
        let half = self.size / 2.0;
        Data {
            vertices: vec![
                [x + half, y + half, z + half],
                [x - half, y + half, z + half],
                [x - half, y - half, z + half],
                [x + half, y - half, z + half],
                [x + half, y + half, z - half],
                [x - half, y + half, z - half],
                [x - half, y - half, z - half],
                [x + half, y - half, z - half],
            ],
            indices: vec![
                [0, 1, 2],
                [0, 2, 3],
                [0, 5, 1],
                [0, 4, 5],
                [0, 7, 4],
                [0, 3, 7],
                [6, 1, 5],
                [6, 2, 1],
                [6, 5, 4],
                [6, 4, 7],
                [6, 7, 3],
                [6, 3, 2],
            ],
        }
    }
}
