use glow::{Context, HasContext};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let canvas: HtmlCanvasElement = document()
        .get_element_by_id("canvas")
        .expect("No canvas")
        .dyn_into()
        .expect("No canvas");

    canvas.set_width(600);
    canvas.set_height(480);

    let webgl2: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .expect("This Platform is unsupported webgl2")
        .expect("No webgl2")
        .dyn_into()
        .expect("No webgl2");

    let gl = Context::from_webgl2_context(webgl2);
    unsafe {
        gl.clear_color(1.0, 0.0, 0.0, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }

    Ok(())
}
