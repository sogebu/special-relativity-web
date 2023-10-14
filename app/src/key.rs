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
