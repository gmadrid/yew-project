#![recursion_limit = "1024"]

mod app;
mod bootstrap;
mod components;
mod gridtrait;
mod meta_grid;
mod other;
mod simplegrid;
mod tablerender;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_twocolor_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}

#[wasm_bindgen]
pub fn run_other_app() -> Result<(), JsValue> {
    yew::start_app::<other::Other>();
    Ok(())
}
