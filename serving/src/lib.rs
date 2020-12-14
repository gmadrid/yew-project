use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_testapp_app() -> Result<(), JsValue> {
    yew::start_app::<testapp::TestApp>();

    Ok(())
}
