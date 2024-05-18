extern crate core;

use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext};

use crate::app::InternalApp;

mod app;
mod charge_set;
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

    pub fn restart_physics(&mut self) {
        self.0.restart_physics();
    }

    pub fn change_c(&mut self, c: f64) -> bool {
        self.0.change_c(c)
    }

    pub fn reset_charge(&mut self, setup: &str) {
        self.0.reset_charge(setup);
    }

    pub fn reset_grid(&mut self, setup: &str) {
        self.0.reset_grid(setup);
    }

    pub fn change_poynting_on(&mut self, poynting_on: bool) {
        self.0.change_poynting_on(poynting_on);
    }

    pub fn change_arrow_length_factor(&mut self, f: f64) {
        self.0.change_arrow_length_factor(f);
    }

    pub fn change_arrow_length_log(&mut self, c: u8) {
        self.0.change_arrow_length_log(c);
    }

    pub fn key_down(&mut self, key: String) {
        self.0.key_down(key);
    }

    pub fn key_up(&mut self, key: String) {
        self.0.key_up(key);
    }

    pub fn window_blue(&mut self) {
        self.0.window_blue();
    }

    pub fn touch_start(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.0.touch_start(ms, x, y);
    }

    pub fn touch_move(&mut self, ms: f64, x: &[f64], y: &[f64]) {
        self.0.touch_move(ms, x, y);
    }

    pub fn touch_end(&mut self, ms: f64) {
        self.0.touch_end(ms);
    }

    pub fn tick(&mut self, timestamp: f64) -> Result<(), JsValue> {
        self.0.tick(timestamp)
    }

    pub fn info(&self) -> String {
        self.0.info()
    }
}
