use crate::key::{KeyManager, TouchManager};
use rmath::{vec3, Deg, Matrix, PhaseSpace, Quaternion, Rad, Vector3, Vector4};

pub struct Player {
    phase_space: PhaseSpace,
    quaternion: Quaternion,
    state: MoveState,
}

enum MoveState {
    Front,
    Back,
    Break,
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
            state: MoveState::Break,
        }
    }

    pub fn tick(&mut self, dt: f64, key: &KeyManager, touch: &TouchManager) {
        let user_input = self.get_user_key_input_acceleration(key)
            + self.calc_user_touch_input_acceleration(touch);
        let a = user_input * 0.5 + self.get_viscous_acceleration() * 0.05;
        self.phase_space.tick(dt, a);
        self.quaternion *= self.get_rotation_velocity(dt, key, touch);
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

    fn get_user_key_input_acceleration(&self, key: &KeyManager) -> Vector3 {
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
            d += self.get_break_acceleration();
        }
        d
    }

    fn calc_user_touch_input_acceleration(&mut self, touch: &TouchManager) -> Vector3 {
        use std::cmp::Ordering;
        let r = touch.pinch_rate().unwrap_or(1.0);
        match self.state {
            MoveState::Front => match r.total_cmp(&1.0) {
                Ordering::Less => {
                    self.state = MoveState::Break;
                    self.get_break_acceleration()
                }
                Ordering::Equal => Vector3::zero(),
                Ordering::Greater => -self.quaternion.front(),
            },
            MoveState::Back => match r.total_cmp(&1.0) {
                Ordering::Less => self.quaternion.front(),
                Ordering::Equal => Vector3::zero(),
                Ordering::Greater => {
                    self.state = MoveState::Break;
                    self.get_break_acceleration()
                }
            },
            MoveState::Break => {
                let is_slow = self.velocity().magnitude2() < 1e-4;
                match r.total_cmp(&1.0) {
                    Ordering::Less => {
                        if is_slow {
                            self.state = MoveState::Back;
                            self.quaternion.front()
                        } else {
                            self.get_break_acceleration()
                        }
                    }
                    Ordering::Equal => Vector3::zero(),
                    Ordering::Greater => {
                        if is_slow {
                            self.state = MoveState::Front;
                            -self.quaternion.front()
                        } else {
                            self.get_break_acceleration()
                        }
                    }
                }
            }
        }
    }

    fn get_break_acceleration(&self) -> Vector3 {
        -self.phase_space.velocity * 10.0
    }

    fn get_viscous_acceleration(&self) -> Vector3 {
        -self.phase_space.velocity
    }

    fn get_rotation_velocity(&self, dt: f64, key: &KeyManager, touch: &TouchManager) -> Quaternion {
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
            if let Some(dxy) = touch.single_move() {
                let mag = dxy.magnitude();
                if mag > 1e-4 {
                    let axis = self.quaternion.up() * -dxy.x + self.quaternion.right() * -dxy.y;
                    return Quaternion::from_axis(Deg(90.0 * mag), axis);
                }
            }
            Quaternion::one()
        } else {
            let axis = self.quaternion.up() * right - self.quaternion.right() * up
                + self.quaternion.front() * role;
            Quaternion::from_axis(Rad(dt), axis)
        }
    }
}
