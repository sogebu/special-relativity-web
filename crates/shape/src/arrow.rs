use crate::{AddFace, Data, Face, VertexPositionCalcNormal};
use rmath::Vector3;

pub struct ArrowOption {
    div: usize,
    root: [f32; 3],
    shaft_length: f32,
    shaft_radius: f32,
    head_length: f32,
    head_radius: f32,
}

impl Default for ArrowOption {
    fn default() -> Self {
        ArrowOption {
            div: 16,
            root: [0.0; 3],
            shaft_length: 2.0,
            shaft_radius: 0.25,
            head_length: 1.0,
            head_radius: 0.5,
        }
    }
}

impl ArrowOption {
    pub fn new() -> ArrowOption {
        Default::default()
    }

    pub fn div(&mut self, div: usize) -> &mut ArrowOption {
        self.div = div;
        self
    }

    pub fn root(&mut self, root: [f32; 3]) -> &mut ArrowOption {
        self.root = root;
        self
    }

    pub fn shaft_length(&mut self, shaft_length: f32) -> &mut ArrowOption {
        self.shaft_length = shaft_length;
        self
    }

    pub fn shaft_radius(&mut self, shaft_radius: f32) -> &mut ArrowOption {
        self.shaft_radius = shaft_radius;
        self
    }

    pub fn head_length(&mut self, head_length: f32) -> &mut ArrowOption {
        self.head_length = head_length;
        self
    }

    pub fn head_radius(&mut self, head_radius: f32) -> &mut ArrowOption {
        self.head_radius = head_radius;
        self
    }

    pub fn build<V>(&self) -> Data<V>
    where
        V: From<VertexPositionCalcNormal>,
        Data<V>: AddFace,
    {
        let [rx, ry, rz] = self.root;

        let calc_xy = |i: usize, r: f32| {
            let theta = std::f64::consts::TAU * (i % self.div) as f64 / self.div as f64;
            let (sin, cos) = theta.sin_cos();
            let x = rx + r * cos as f32;
            let y = ry + r * sin as f32;
            (x, y)
        };

        // bottom
        let mut vertices = Vec::with_capacity(1 + self.div * 3);
        let mut triangles = Vec::with_capacity(self.div * 6);
        let normal = Vector3::new(0.0, 0.0, -1.0);
        vertices.push(VertexPositionCalcNormal {
            position: [rx, ry, rz],
            normal,
        });
        for i in 0..self.div {
            let (x, y) = calc_xy(i, self.shaft_radius);
            vertices.push(VertexPositionCalcNormal {
                position: [x, y, rz],
                normal,
            });
        }
        for i in 0..self.div {
            triangles.push([0, 1 + ((i + 1) % self.div) as u32, 1 + i as u32]);
        }
        let mut bottom = Data {
            vertices,
            triangles,
        };

        // side of shaft
        let mut shaft_side =
            Data::<VertexPositionCalcNormal>::with_capacity(self.div * 2, self.div * 2);
        for i in 0..self.div {
            let (x1, y1) = calc_xy(i, self.shaft_radius);
            let (x2, y2) = calc_xy(i + 1, self.shaft_radius);
            let z1 = rz;
            let z2 = rz + self.shaft_length;
            shaft_side.add_face(&Face {
                vertices: vec![[x1, y1, z1], [x2, y2, z1], [x2, y2, z2], [x1, y1, z2]],
            });
        }
        bottom.append(shaft_side);

        // bottom of head
        let mut vertices = Vec::with_capacity(self.div * 2);
        let mut triangles = Vec::with_capacity(self.div * 2);
        let normal = Vector3::new(0.0, 0.0, -1.0);
        for i in 0..self.div {
            let (x1, y1) = calc_xy(i, self.shaft_radius);
            let (x2, y2) = calc_xy(i, self.head_radius);
            vertices.push(VertexPositionCalcNormal {
                position: [x1, y1, rz + self.shaft_length],
                normal,
            });
            vertices.push(VertexPositionCalcNormal {
                position: [x2, y2, rz + self.shaft_length],
                normal,
            });

            let i1 = i as u32 * 2;
            let j1 = i as u32 * 2 + 1;
            let i2 = ((i + 1) % self.div) as u32 * 2;
            let j2 = ((i + 1) % self.div) as u32 * 2 + 1;
            triangles.push([i1, i2, j2]);
            triangles.push([i1, j2, j1]);
        }
        let head_bottom = Data {
            vertices,
            triangles,
        };
        bottom.append(head_bottom);

        // head
        let o = [rx, ry, rz + self.shaft_length + self.head_length];
        let mut head = Data::<VertexPositionCalcNormal>::with_capacity(self.div + 1, self.div);
        for i in 0..self.div {
            let (x1, y1) = calc_xy(i, self.head_radius);
            let (x2, y2) = calc_xy(i + 1, self.head_radius);
            let z = rz + self.shaft_length;
            head.add_face(&Face {
                vertices: vec![[x1, y1, z], o, [x2, y2, z]],
            });
        }
        bottom.append(head);

        bottom.vertex_converted()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{VertexPosition, VertexPositionCalcNormal, VertexPositionNormal};

    #[test]
    fn no_normal() {
        let cube = ArrowOption::new().div(5).build::<VertexPosition>().dedup();
        assert_eq!(cube.vertices.len(), 1 + 5 * 3 + 1);
        assert_eq!(cube.triangles.len(), 5 * 6);
    }

    #[test]
    fn face_normal() {
        let cube = ArrowOption::new().div(6).build::<VertexPositionNormal>();
        assert_eq!(cube.vertices.len(), 1 + 6 * 6 + 1);
        assert_eq!(cube.triangles.len(), 6 * 6);
    }

    #[test]
    fn vert_normal() {
        let cube = ArrowOption::new()
            .div(7)
            .build::<VertexPositionCalcNormal>();
        assert_eq!(cube.vertices.len(), 1 + 7 * 6 + 1);
        assert_eq!(cube.triangles.len(), 7 * 6);
    }
}
