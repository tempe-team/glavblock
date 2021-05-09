#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod core;
mod production;
mod resources;
mod storage;
mod people;
mod area;
mod turn;
mod assets;
mod queries;
mod events;

pub use app::GlavblockApp;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// ----------------------------------------------------------------------------
// When compiling for web:
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    init_panic_hook();
    let app = GlavblockApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
