#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
mod core;
mod production;
mod resources;
mod storage;
mod people;
mod area;
mod turn;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = glavblock::GlavblockApp::default();
    eframe::run_native(Box::new(app));
}
