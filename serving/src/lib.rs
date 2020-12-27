use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_crown_app() -> Result<(), JsValue> {
    yew::start_app::<crown::CrownApp>();

    Ok(())
}

#[wasm_bindgen]
pub fn run_metapixel_app() -> Result<(), JsValue> {
    yew::start_app::<metapixel::MetapixelApp>();

    Ok(())
}

#[wasm_bindgen]
pub fn run_testapp_app() -> Result<(), JsValue> {
    yew::start_app::<testapp::TestApp>();

    Ok(())
}
