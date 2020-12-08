#![recursion_limit = "1024"]

mod apps;
mod bootstrap;
mod components;
mod grids;
mod meta_grid;
mod simplegrid;
mod tablerender;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_twocolor_app() -> Result<(), JsValue> {
    yew::start_app::<apps::TwoPattern>();

    Ok(())
}

#[wasm_bindgen]
pub fn run_metapixel_app() -> Result<(), JsValue> {
    yew::start_app::<apps::Metapixel>();
    Ok(())
}

#[wasm_bindgen]
pub fn run_tiles_app() -> Result<(), JsValue> {
    yew::start_app::<apps::Tiles>();
    Ok(())
}
