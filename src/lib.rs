#![recursion_limit = "1024"]

mod app;
mod bootstrap;
mod gridtrait;
mod simplegrid;
mod tablerender;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
