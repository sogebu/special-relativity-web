use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use crate::app::InternalApp;

mod app;
mod backend;
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
    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        self.0.tick(timestamp)
    }
}
