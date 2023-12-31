use crate::Data;

#[derive(Debug)]
pub struct IcosahedronOption {
    radius: f32,
    center: [f32; 3],
}

impl Default for IcosahedronOption {
    fn default() -> Self {
        IcosahedronOption {
            radius: 1.0,
            center: [0.0; 3],
        }
    }
}

impl IcosahedronOption {
    pub fn new() -> IcosahedronOption {
        Default::default()
    }

    pub fn radius(&mut self, radius: f32) -> &mut IcosahedronOption {
        self.radius = radius;
        self
    }

    pub fn center(&mut self, center: [f32; 3]) -> &mut IcosahedronOption {
        self.center = center;
        self
    }

    /// Ref: https://en.wikipedia.org/wiki/Regular_icosahedron
    pub fn build(&self) -> Data<[f32; 3]> {
        let [x, y, z] = self.center;
        let r = self.radius;
        let phi = (1.0 + 5.0f64.sqrt()) / 2.0;
        let a = ((r as f64) / (phi * phi + 1.0).sqrt()) as f32;
        let b = ((r as f64) * phi / (phi * phi + 1.0).sqrt()) as f32;
        Data {
            vertices: vec![
                [x, y + a, z + b], // 0
                [x, y + a, z - b], // 1
                [x, y - a, z + b], // 2
                [x, y - a, z - b], // 3
                [x + b, y, z + a], // 4
                [x - b, y, z + a], // 5
                [x + b, y, z - a], // 6
                [x - b, y, z - a], // 7
                [x + a, y + b, z], // 8
                [x + a, y - b, z], // 9
                [x - a, y + b, z], // 10
                [x - a, y - b, z], // 11
            ],
            triangles: vec![
                [0, 2, 4],
                [2, 0, 5],
                [3, 1, 6],
                [1, 3, 7],
                [4, 6, 8],
                [6, 4, 9],
                [7, 5, 10],
                [5, 7, 11],
                [8, 10, 0],
                [10, 8, 1],
                [11, 9, 2],
                [9, 11, 3],
                [0, 4, 8],
                [4, 2, 9],
                [2, 5, 11],
                [5, 0, 10],
                [1, 8, 6],
                [6, 9, 3],
                [3, 11, 7],
                [7, 10, 1],
            ],
        }
    }
}
