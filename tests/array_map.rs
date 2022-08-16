use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::FluvioError;

#[wasm_bindgen(module = "/tests/js/utils.js")]
extern "C" {
    pub fn createUUID() -> String;
}

#[wasm_bindgen_test]
async fn array_map() {
    #[wasm_bindgen(module = "/tests/js/array_map/array_map.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }

    #[wasm_bindgen(module = "/tests/js/array_map/array_map_code.js")]
    extern "C" {
        static ARRAY_MAP: String;
    }
    web_sys::console::time_with_label("array-map-suite");

    web_sys::console::time_with_label("array-map-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("array-map-setup");

    web_sys::console::time_with_label("array-map-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("array-map-test");

    web_sys::console::time_with_label("array-map-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("array-map-teardown");

    web_sys::console::time_end_with_label("array-map-suite");
}
