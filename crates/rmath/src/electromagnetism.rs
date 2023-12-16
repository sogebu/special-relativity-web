use crate::{Matrix, Vector3};

impl Matrix {
    /// Calculate field strength
    ///
    /// q: charge
    /// l: position vector, from observer to charge, in observer's inertial frame
    /// u: covariant velocity of charge on observer's PLC in observer's inertial frame
    /// a: covariant acceleration of charge on observer's PLC in observer's inertial frame
    pub fn field_strength(q: f64, l: Vector3, u: Vector3, a: Vector3) -> Matrix {
        let l_len = l.magnitude();
        if l_len < f64::EPSILON * 2.0 {
            // too near
            return Matrix::zero();
        }
        let l_hat = l / l_len;
        let u_t = u.gamma();
        let a_t = a.dot(u) / u_t;

        let lu = l_hat.dot(u);
        let la = l_hat.dot(a);
        let term_1 = l_hat * ((u_t * (la - 1.0 / l_len) - a_t * lu) / u_t.powi(3));
        let term_2 = u * ((a_t + la - 1.0 / l_len) / u_t.powi(3));
        let term_3 = a / u_t.powi(2);
        let f_t = term_1 + term_2 - term_3;

        let t_1 = 1.0 / u_t.powi(2);
        let t_2 = (a_t + la - 1.0 / l_len) / u_t.powi(3);
        let f_xy = (l_hat.x * a.y - l_hat.y * a.x) * t_1 - (l_hat.x * u.y - l_hat.y * u.x) * t_2;
        let f_yz = (l_hat.y * a.z - l_hat.z * a.y) * t_1 - (l_hat.y * u.z - l_hat.z * u.y) * t_2;
        let f_zx = (l_hat.z * a.x - l_hat.x * a.z) * t_1 - (l_hat.z * u.x - l_hat.x * u.z) * t_2;

        Matrix::new(
            [0.0, f_xy, -f_zx, f_t.x],
            [-f_xy, 0.0, f_yz, f_t.y],
            [f_zx, -f_yz, 0.0, f_t.z],
            [-f_t.x, -f_t.y, -f_t.z, 0.0],
        ) * (q / l_len)
    }

    pub fn field_strength_to_electric_field(&self) -> Vector3 {
        Vector3::new(self.rows[0][3], self.rows[1][3], self.rows[2][3])
    }

    pub fn field_strength_to_magnetic_field(&self) -> Vector3 {
        Vector3::new(
            self.rows[1][2] - self.rows[2][1],
            self.rows[2][0] - self.rows[0][2],
            self.rows[0][1] - self.rows[1][0],
        )
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn static_charge() {
        let l = Vector3::new(3.0, 4.0, 5.0);
        let fs = Matrix::field_strength(2.0, l, Vector3::zero(), Vector3::zero());
        let l_len = l.magnitude();
        assert_relative_eq!(
            fs.field_strength_to_electric_field(),
            -l.normalized() * 2.0 / l_len / l_len,
        );
        assert_relative_eq!(fs.field_strength_to_magnetic_field(), Vector3::zero());
    }

    #[test]
    fn moving_charge() {
        let l = Vector3::new(3.0, 4.0, 5.0);
        let u = Vector3::new(0.1, 0.2, -0.05);
        let a = Vector3::new(0.01, 0.02, 0.03);
        let fs = Matrix::field_strength(2.0, l, u, a);
        let e = fs.field_strength_to_electric_field();
        let m = fs.field_strength_to_magnetic_field();
        assert!(e.magnitude() > 0.0);
        assert!(m.magnitude() > 0.0);
        assert_relative_eq!(e.dot(m), 0.0);
    }
}
