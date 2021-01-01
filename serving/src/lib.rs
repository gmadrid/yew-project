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

app_runner!(run_crown_app, crown::CrownApp);
app_runner!(run_metapixel_app, metapixel::MetapixelApp);
app_runner!(run_testapp_app, testapp::TestApp);
app_runner!(run_tiles_app, tiles::TilesApp);
app_runner!(run_twocolor_app, twocolor::TwoColorApp);
