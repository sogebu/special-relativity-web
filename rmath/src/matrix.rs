use std::ops::Mul;

use bytemuck::{Pod, Zeroable};

use crate::vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct Matrix {
    cols: [[f32; 4]; 4],
}

impl Matrix {
    #[allow(clippy::too_many_arguments)]
    #[rustfmt::skip]
    pub const fn new(
        r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
        r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
        r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
        r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
    ) -> Matrix {
        Matrix {
            cols: [
                [r0c0, r1c0, r2c0, r3c0],
                [r0c1, r1c1, r2c1, r3c1],
                [r0c2, r1c2, r2c2, r3c2],
                [r0c3, r1c3, r2c3, r3c3],
            ],
        }
    }

    #[rustfmt::skip]
    pub const fn ident() -> Matrix {
        Matrix::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut t = [[0.0; 4]; 4];
        for (c, t) in t.iter_mut().enumerate() {
            for (r, t) in t.iter_mut().enumerate() {
                for i in 0..4 {
                    *t += self.cols[i][r] as f64 * rhs.cols[c][i] as f64;
                }
            }
        }
        Matrix::new(
            t[0][0] as f32,
            t[1][0] as f32,
            t[2][0] as f32,
            t[3][0] as f32,
            t[0][1] as f32,
            t[1][1] as f32,
            t[2][1] as f32,
            t[3][1] as f32,
            t[0][2] as f32,
            t[1][2] as f32,
            t[2][2] as f32,
            t[3][2] as f32,
            t[0][3] as f32,
            t[1][3] as f32,
            t[2][3] as f32,
            t[3][3] as f32,
        )
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self::Output {
        let mut t = [0.0; 3];
        for (i, t) in t.iter_mut().enumerate() {
            *t = self.cols[0][i] as f64 * v.x as f64
                + self.cols[1][i] as f64 * v.y as f64
                + self.cols[2][i] as f64 * v.z as f64
                + self.cols[3][i] as f64;
        }
        Vector3::new(t[0] as f32, t[1] as f32, t[2] as f32)
    }
}
