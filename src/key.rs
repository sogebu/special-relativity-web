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
    Single { x: f64, y: f64 },
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

    pub fn touch_start(&mut self, x: &[f64], y: &[f64]) {
        if x.len() == 1 {
            self.pre_state = self.current_state;
            self.current_state = TouchState::Single { x: x[0], y: y[0] };
            self.last_event = self.current_state;
        }
    }

    pub fn touch_move(&mut self, x: &[f64], y: &[f64]) {
        if x.len() == 1 {
            self.pre_state = self.current_state;
            self.current_state = TouchState::Single { x: x[0], y: y[0] };
            self.last_event = self.current_state;
        }
    }

    pub fn touch_end(&mut self) {
        self.pre_state = self.current_state;
        self.current_state = TouchState::None;
        self.last_event = TouchState::None;
    }

    pub fn tick(&mut self) {
        if self.last_event == self.current_state {
            self.pre_state = self.current_state;
        }
    }

    pub fn single_move(&self) -> Option<(f64, f64)> {
        let TouchState::Single { x: pre_x, y: pre_y } = self.pre_state else {
            return None;
        };
        let TouchState::Single {
            x: current_x,
            y: current_y,
        } = self.current_state
        else {
            return None;
        };
        let dx = (current_x - pre_x) / self.width;
        let dy = (current_y - pre_y) / self.height;
        Some((dx, dy))
    }
}
