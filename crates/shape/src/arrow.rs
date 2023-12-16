use crate::Data;

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

    pub fn build(&self) -> Data {
        let [rx, ry, rz] = self.root;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let calc_xy = |i: usize, r: f32| {
            let theta = std::f64::consts::TAU * i as f64 / self.div as f64;
            let (sin, cos) = theta.sin_cos();
            let x = rx + r * sin as f32;
            let y = ry + r * cos as f32;
            (x, y)
        };

        vertices.push([rx, ry, rz]);
        for i in 0..self.div {
            let (x, y) = calc_xy(i, self.shaft_radius);
            vertices.push([x, y, rz]);
        }
        // face of bottom
        for i in 0..self.div {
            indices.push([0, 1 + ((i + 1) % self.div) as u32, 1 + i as u32]);
        }

        for i in 0..self.div {
            let (x, y) = calc_xy(i, self.shaft_radius);
            vertices.push([x, y, rz + self.shaft_length]);
        }
        // face of shaft side
        for i in 0..self.div {
            let a = 1 + i as u32;
            let b = 1 + ((i + 1) % self.div) as u32;
            let c = 1 + (self.div + i) as u32;
            let d = 1 + (self.div + (i + 1) % self.div) as u32;
            indices.push([a, b, d]);
            indices.push([d, c, a]);
        }

        for i in 0..self.div {
            let (x, y) = calc_xy(i, self.head_radius);
            vertices.push([x, y, rz + self.shaft_length]);
        }
        vertices.push([rx, ry, rz + self.shaft_length + self.head_length]);

        // face of head
        for i in 0..self.div {
            let a = 1 + (self.div + i) as u32;
            let b = 1 + (self.div + (i + 1) % self.div) as u32;
            let c = 1 + (self.div * 2 + i) as u32;
            let d = 1 + (self.div * 2 + (i + 1) % self.div) as u32;
            let e = 1 + (self.div * 3) as u32;
            indices.push([a, b, d]);
            indices.push([d, c, a]);
            indices.push([c, d, e]);
        }

        Data { vertices, indices }
    }
}
