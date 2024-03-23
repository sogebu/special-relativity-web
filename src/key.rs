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
    pub touch_events: Vec<(f64, TouchEvent)>,
    gesture_event: Vec<(f64, GestureEvent)>,
}

#[derive(Copy, Clone)]
pub enum TouchEvent {
    SingleStart(Vector2),
    SingleMove(Vector2),
    DoubleStart(Vector2, Vector2),
    DoubleMove(Vector2, Vector2),
    End,
}

#[derive(Copy, Clone)]
pub enum GestureEvent {
    Swipe(Vector2),
    DoubleTap,
    Pinch(f64),
}

impl TouchManager {
    pub fn new(width: f64, height: f64) -> TouchManager {
        TouchManager {
            width,
            height,
            touch_events: Vec::new(),
            gesture_event: Vec::new(),
        }
    }

    fn push_touch_event(&mut self, ms: f64, event: TouchEvent) {
        self.touch_events.push((ms, event));
        if self.touch_events.len() >= 5 {
            self.touch_events.remove(0);
        }
    }

    fn push_gesture_event(&mut self, ms: f64, event: GestureEvent) {
        self.gesture_event.push((ms, event));
        if self.gesture_event.len() >= 5 {
            self.gesture_event.remove(0);
        }
    }

    pub fn touch_start(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        if x.len() == 1 {
            self.push_touch_event(ms, TouchEvent::SingleStart(Vector2::new(x[0], y[0])));
        } else if x.len() == 2 {
            self.push_touch_event(
                ms,
                TouchEvent::DoubleStart(Vector2::new(x[0], y[0]), Vector2::new(x[1], y[1])),
            );
        }
        self.calc_gesture();
    }

    pub fn touch_move(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        if x.len() == 1 {
            let v = Vector2::new(x[0], y[0]);
            match self.touch_events.last().copied() {
                None => {}
                Some((_, TouchEvent::SingleStart(w))) | Some((_, TouchEvent::SingleMove(w))) => {
                    if v == w {
                        return;
                    }
                }
                Some(_) => {}
            }
            self.push_touch_event(ms, TouchEvent::SingleMove(v));
        } else if x.len() == 2 {
            self.push_touch_event(
                ms,
                TouchEvent::DoubleMove(Vector2::new(x[0], y[0]), Vector2::new(x[1], y[1])),
            );
        }
        self.calc_gesture();
    }

    pub fn touch_end(&mut self, ms: f64) {
        self.push_touch_event(ms, TouchEvent::End);
        self.calc_gesture();
    }

    fn calc_gesture(&mut self) {
        use TouchEvent::*;
        match self.touch_events.as_slice() {
            [.., (_, SingleMove(pre)), (ts, SingleMove(current))]
            | [.., (_, SingleStart(pre)), (ts, SingleMove(current))] => {
                let dx = (current.x - pre.x) / self.width;
                let dy = (current.y - pre.y) / self.height;
                self.push_gesture_event(*ts, GestureEvent::Swipe(Vector2::new(dx, dy)));
            }

            [.., (_, DoubleMove(pre1, pre2)), (ts, DoubleMove(cur1, cur2))]
            | [.., (_, DoubleStart(pre1, pre2)), (ts, DoubleMove(cur1, cur2))] => {
                let pre = (*pre2 - *pre1).magnitude();
                let cur = (*cur2 - *cur1).magnitude();
                self.push_gesture_event(*ts, GestureEvent::Pinch(cur / pre));
            }

            [(t1, SingleStart(p1)), (t2, End), (t3, SingleStart(p2)), (t4, End)] => {
                const TAP_TH: f64 = 300.0;
                const DOUBLE_TAP_TIME_TH: f64 = 500.0;
                const DOUBLE_TAP_DIST_TH: f64 = 100.0;
                if *t2 - *t1 < TAP_TH
                    && *t4 - *t3 < TAP_TH
                    && *t4 - *t2 < DOUBLE_TAP_TIME_TH
                    && (*p2 - *p1).magnitude() < DOUBLE_TAP_DIST_TH
                {
                    self.push_gesture_event(*t4, GestureEvent::DoubleTap);
                }
            }
            _ => (),
        }
    }

    pub fn consume_gesture(&mut self, ts: f64) -> Option<GestureEvent> {
        if !self.gesture_event.is_empty() {
            let (ms, event) = self.gesture_event.remove(0);
            if ts - ms <= 100.0 {
                return Some(event);
            }
        }
        None
    }
}
