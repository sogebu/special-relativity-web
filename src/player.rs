use crate::key::{GestureEvent, KeyManager};
use rmath::{Deg, Matrix, PhaseSpace, Quaternion, Rad, Vector2, Vector3, Vector4};

pub struct Player {
    phase_space: PhaseSpace,
    quaternion: Quaternion,
    breaking: bool,
}

impl Player {
    pub fn new(x: Vector3) -> Player {
        Player {
            phase_space: PhaseSpace::new(Vector3::zero(), Vector4::from_tv(0.0, x)),
            quaternion: Quaternion::one(),
            breaking: false,
        }
    }

    pub fn tick(&mut self, c: f64, dt: f64, key: &KeyManager, gestures: &[GestureEvent]) {
        let user_input = self.get_user_key_input_acceleration(key)
            + self.get_user_gesture_acceleration(gestures);
        if user_input == Vector3::zero() {
            if key.is_pressed("r") || gesture_double_tap(gestures) {
                self.breaking = true;
            }
        } else {
            self.breaking = false;
        }
        let f_over_m = user_input * 0.5
            + self.get_viscous_acceleration() * if self.breaking { 3.0 } else { 0.0 };
        let a = f_over_m / c / c;
        let ds = dt * c;
        self.phase_space.tick(ds, a);

        if let Some(q) = self.get_user_key_input_rotation_velocity(dt, key) {
            self.quaternion *= q;
        }
        if let Some(q) = self.get_user_gesture_rotation_velocity(gestures) {
            self.quaternion *= q;
        }
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
        d.safe_normalized()
    }

    fn get_user_gesture_acceleration(&self, gestures: &[GestureEvent]) -> Vector3 {
        use std::cmp::Ordering;
        if let Some(r) = gesture_pinch(gestures) {
            match r.total_cmp(&1.0) {
                // back
                Ordering::Less => self.quaternion.front(),
                // not move
                Ordering::Equal => Vector3::zero(),
                // forward
                Ordering::Greater => -self.quaternion.front(),
            }
        } else {
            Vector3::zero()
        }
    }

    fn get_viscous_acceleration(&self) -> Vector3 {
        -self.phase_space.velocity
    }

    fn get_user_key_input_rotation_velocity(
        &self,
        dt: f64,
        key: &KeyManager,
    ) -> Option<Quaternion> {
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
            None
        } else {
            let axis = self.quaternion.up() * right - self.quaternion.right() * up
                + self.quaternion.front() * role;
            Some(Quaternion::from_axis(Rad(dt), axis))
        }
    }

    fn get_user_gesture_rotation_velocity(&self, gestures: &[GestureEvent]) -> Option<Quaternion> {
        if let Some(dxy) = gesture_swipe(gestures) {
            let mag = dxy.magnitude();
            if mag > 1e-4 {
                let axis = self.quaternion.up() * -dxy.x + self.quaternion.right() * -dxy.y;
                return Some(Quaternion::from_axis(Deg(90.0 * mag), axis));
            }
        }
        None
    }
}

fn gesture_swipe(gestures: &[GestureEvent]) -> Option<Vector2> {
    for &gesture in gestures {
        match gesture {
            GestureEvent::Swipe(v) => {
                return Some(v);
            }
            GestureEvent::DoubleTap => {}
            GestureEvent::Pinch(_) => {}
        }
    }
    None
}

fn gesture_double_tap(gestures: &[GestureEvent]) -> bool {
    for &gesture in gestures {
        match gesture {
            GestureEvent::Swipe(_) => {}
            GestureEvent::DoubleTap => return true,
            GestureEvent::Pinch(_) => {}
        }
    }
    false
}

fn gesture_pinch(gestures: &[GestureEvent]) -> Option<f64> {
    for &gesture in gestures {
        match gesture {
            GestureEvent::Swipe(_) => {}
            GestureEvent::DoubleTap => {}
            GestureEvent::Pinch(r) => {
                return Some(r);
            }
        }
    }
    None
}
