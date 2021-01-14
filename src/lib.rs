#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;

macro_rules! app_runner {
    ($func:ident, $app_class:path) => {
        #[wasm_bindgen]
        pub fn $func() -> Result<(), JsValue> {
            yew::start_app::<$app_class>();
            Ok(())
        }
    };
}

mod PROJECT;

pub use crate::PROJECT::PROJECTApp;

app_runner!(run_PROJECT_app, PROJECT::PROJECTApp);
