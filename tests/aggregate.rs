use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

//use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::FluvioError;

#[wasm_bindgen(module = "/tests/js/utils.js")]
extern "C" {
    pub fn createUUID() -> String;
}
/*
#[wasm_bindgen_test]
async fn aggreegate() {
    #[wasm_bindgen(module = "/tests/js/aggregate/aggregate.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    #[wasm_bindgen(module = "/tests/js/aggregate/aggregate_code.js")]
    extern "C" {
        type MarkerForWasmBindgen;
    }

    web_sys::console::time_with_label("smartstream-aggregate-suite");
    web_sys::console::time_with_label("smartstream-aggregate-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("smartstream-aggregate-setup");
    web_sys::console::time_with_label("smartstream-aggregate-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("smartstream-aggregate-test");
    web_sys::console::time_with_label("smartstream-aggregate-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("smartstream-aggregate-teardown");
    web_sys::console::time_end_with_label("smartstream-aggregate-suite");
}
*/
