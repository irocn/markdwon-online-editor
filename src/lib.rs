#![recursion_limit = "512"]

extern crate pulldown_cmark;
//extern crate stdweb;

mod app;
mod utils;

use log::info;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {

    info!("starting...");


    utils::set_panic_hook();
    web_logger::init();
    yew::start_app::<app::App>();
    Ok(())
}
