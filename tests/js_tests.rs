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
async fn simple() {
    #[wasm_bindgen(module = "/tests/js/simple/simple.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    web_sys::console::time_with_label("simple-suite");
    web_sys::console::time_with_label("simple-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("simple-setup");
    web_sys::console::time_with_label("simple-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("simple-test");
    web_sys::console::time_with_label("simple-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("simple-teardown");
    web_sys::console::time_end_with_label("simple-suite");
}
/*
*/

#[wasm_bindgen_test]
async fn connector() {
    #[wasm_bindgen(module = "/tests/js/connector/connector.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    web_sys::console::time_with_label("connector-suite");
    web_sys::console::time_with_label("connector-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("connector-setup");
    web_sys::console::time_with_label("connector-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("connector-test");
    web_sys::console::time_with_label("connector-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("connector-teardown");
    web_sys::console::time_end_with_label("connector-suite");
}

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

/*
#[wasm_bindgen_test]
async fn smartstream_filter() {
    #[wasm_bindgen(module = "/tests/js/smartstream_filter/smartstream_filter.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    #[wasm_bindgen(module = "/tests/js/smartstream_filter/smartstream_filter_code.js")]
    extern "C" {
        type MarkerForWasmBindgen;
    }

    web_sys::console::time_with_label("smartstream-filter-suite");
    web_sys::console::time_with_label("smartstream-filter-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("smartstream-filter-setup");
    web_sys::console::time_with_label("smartstream-filter-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("smartstream-filter-test");
    web_sys::console::time_with_label("smartstream-filter-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("smartstream-filter-teardown");
    web_sys::console::time_end_with_label("smartstream-filter-suite");
}
*/
