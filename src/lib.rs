use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use crate::app::InternalApp;

mod app;
mod key;
mod player;

#[allow(dead_code)]
fn log(s: String) {
    console::log_1(&s.into());
}

#[wasm_bindgen]
pub struct App(InternalApp);

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(context: WebGl2RenderingContext) -> Result<App, JsValue> {
        Ok(App(InternalApp::new(context)?))
    }

    pub fn reset_charge(&mut self, setup: &str) {
        self.0.reset_charge(setup);
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: String) {
        self.0.key_down(key);
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self, key: String) {
        self.0.key_up(key);
    }

    #[wasm_bindgen]
    pub fn window_blue(&mut self) {
        self.0.window_blue();
    }

    #[wasm_bindgen]
    pub fn touch_start(&mut self, x: &[f64], y: &[f64]) {
        self.0.touch_start(x, y);
    }

    #[wasm_bindgen]
    pub fn touch_move(&mut self, x: &[f64], y: &[f64]) {
        self.0.touch_move(x, y);
    }

    #[wasm_bindgen]
    pub fn touch_end(&mut self) {
        self.0.touch_end();
    }

    #[wasm_bindgen]
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        self.0.tick(timestamp)
    }

    #[wasm_bindgen]
    pub fn info(&self) -> String {
        self.0.info()
    }
}
