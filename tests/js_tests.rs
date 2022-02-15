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

#[wasm_bindgen_test]
async fn multiple_partitions() {
    #[wasm_bindgen(module = "/tests/js/multiple_partitions/multiple_partitions.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    web_sys::console::time_with_label("multiple-partitions-suite");
    web_sys::console::time_with_label("multiple-partitions-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("multiple-partitions-setup");
    web_sys::console::time_with_label("multiple-partitions-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("multiple-partitions-test");
    web_sys::console::time_with_label("multiple-partitions-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("multiple-partitions-teardown");
    web_sys::console::time_end_with_label("multiple-partitions-suite");
}

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
