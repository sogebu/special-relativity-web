use crate::key::KeyManager;
use rmath::{vec3, Matrix, PhaseSpace, Quaternion, Rad, Vector3, Vector4};

pub struct Player {
    phase_space: PhaseSpace,
    quaternion: Quaternion,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            phase_space: PhaseSpace::new(
                Vector3::zero(),
                Vector4::from_tv(0.0, vec3(0.0, 0.0, 10.0)),
            ),
            quaternion: Quaternion::one(),
        }
    }

    pub fn tick(&mut self, dt: f64, key: &KeyManager) {
        let a =
            self.get_user_input_acceleration(key) * 0.5 + self.get_viscous_acceleration() * 0.05;
        self.phase_space.tick(dt, a);
        self.quaternion *= self.get_rotation_velocity(dt, key);
    }

    pub fn rot_matrix(&self) -> Matrix {
        Matrix::from(self.quaternion)
    }

    pub fn inv_rot_matrix(&self) -> Matrix {
        Matrix::from(self.quaternion.inverse())
    }

    pub fn lorentz_matrix(&self) -> Matrix {
        Matrix::lorentz(self.phase_space.velocity)
    }

    pub fn position(&self) -> Vector4 {
        self.phase_space.position
    }

    pub fn velocity(&self) -> Vector3 {
        self.phase_space.velocity
    }

    fn get_user_input_acceleration(&self, key: &KeyManager) -> Vector3 {
        let mut d = Vector3::zero();
        // forward
        if key.is_pressed("w") {
            d -= self.quaternion.front()
        }
        if key.is_pressed("s") {
            d += self.quaternion.front()
        }
        // right-left
        if key.is_pressed("d") {
            d += self.quaternion.right();
        }
        if key.is_pressed("a") {
            d -= self.quaternion.right();
        }
        // up-down
        if key.is_pressed("z") {
            d -= self.quaternion.up();
        }
        if key.is_pressed("x") {
            d += self.quaternion.up();
        }
        d = d.safe_normalized();
        // break
        if key.is_pressed("r") {
            d -= self.phase_space.velocity * 10.0;
        }
        d
    }

    fn get_viscous_acceleration(&self) -> Vector3 {
        -self.phase_space.velocity
    }

    fn get_rotation_velocity(&self, dt: f64, key: &KeyManager) -> Quaternion {
        let mut right = 0.0;
        if key.is_pressed("arrowright") {
            right += 1.0;
        }
        if key.is_pressed("arrowleft") {
            right -= 1.0;
        }
        let mut up = 0.0;
        if key.is_pressed("arrowup") {
            up += 1.0;
        }
        if key.is_pressed("arrowdown") {
            up -= 1.0;
        }
        let mut role = 0.0;
        if key.is_pressed("e") {
            role += 1.0;
        }
        if key.is_pressed("q") {
            role -= 1.0;
        }
        if (right, up, role) == (0.0, 0.0, 0.0) {
            Quaternion::one()
        } else {
            let axis = self.quaternion.up() * right - self.quaternion.right() * up
                + self.quaternion.front() * role;
            Quaternion::from_axis(Rad(dt), axis)
        }
    }
}
