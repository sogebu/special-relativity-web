use rmath::Vector2;
use std::collections::HashSet;

pub struct KeyManager {
    pressed: HashSet<String>,
}

impl Default for KeyManager {
    fn default() -> Self {
        KeyManager::new()
    }
}

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager {
            pressed: HashSet::new(),
        }
    }

    pub fn down(&mut self, key: String) {
        self.pressed.insert(key);
    }

    pub fn up(&mut self, key: String) {
        self.pressed.remove(&key);
    }

    pub fn clear(&mut self) {
        self.pressed = HashSet::new();
    }

    pub fn is_pressed(&self, key: &str) -> bool {
        self.pressed.contains(key)
    }
}

pub struct TouchManager {
    width: f64,
    height: f64,
    last_event: TouchState,
    pre_state: TouchState,
    current_state: TouchState,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TouchState {
    None,
    Single(Vector2),
    Double(Vector2, Vector2),
}

impl TouchManager {
    pub fn new(width: f64, height: f64) -> TouchManager {
        TouchManager {
            width,
            height,
            last_event: TouchState::None,
            pre_state: TouchState::None,
            current_state: TouchState::None,
        }
    }

    fn push(&mut self, s: TouchState) {
        self.pre_state = self.current_state;
        self.current_state = s;
        self.last_event = s;
    }

    pub fn touch_move(&mut self, x: &[f64], y: &[f64]) {
        if x.len() == 1 {
            self.push(TouchState::Single(Vector2::new(x[0], y[0])));
        } else if x.len() == 2 {
            self.push(TouchState::Double(
                Vector2::new(x[0], y[0]),
                Vector2::new(x[1], y[1]),
            ));
        }
    }

    pub fn touch_end(&mut self) {
        self.push(TouchState::None);
    }

    pub fn tick(&mut self) {
        if self.last_event == self.current_state {
            self.pre_state = self.current_state;
        }
    }

    pub fn single_move(&self) -> Option<Vector2> {
        let TouchState::Single(pre) = self.pre_state else {
            return None;
        };
        let TouchState::Single(current) = self.current_state else {
            return None;
        };
        let dx = (current.x - pre.x) / self.width;
        let dy = (current.y - pre.y) / self.height;
        Some(Vector2::new(dx, dy))
    }

    pub fn pinch_rate(&self) -> Option<f64> {
        let TouchState::Double(pre1, pre2) = self.pre_state else {
            return None;
        };
        let TouchState::Double(current1, current2) = self.current_state else {
            return None;
        };
        let pre = (pre2 - pre1).magnitude();
        let current = (current2 - current1).magnitude();
        Some(current / pre)
    }
}
