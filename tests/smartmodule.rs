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

#[wasm_bindgen_test]
async fn smartstream_map() {
    #[wasm_bindgen(module = "/tests/js/smartstream_map/map.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }
    #[wasm_bindgen(module = "/tests/js/smartstream_map/map_code.js")]
    extern "C" {
        static MAP_CODE: String;
    }

    web_sys::console::time_with_label("smartstream-map-suite");
    web_sys::console::time_with_label("smartstream-map-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("smartstream-map-setup");
    web_sys::console::time_with_label("smartstream-map-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("smartstream-map-test");
    web_sys::console::time_with_label("smartstream-map-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("smartstream-map-teardown");
    web_sys::console::time_end_with_label("smartstream-map-suite");
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

#[wasm_bindgen_test]
async fn filter_map() {
    #[wasm_bindgen(module = "/tests/js/filter_map/filter_map.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn setup() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown() -> Result<JsValue, JsValue>;
    }

    #[wasm_bindgen(module = "/tests/js/filter_map/filter_map_code.js")]
    extern "C" {
        static FILTER_MAP: String;
    }
    web_sys::console::time_with_label("filter-map-suite");

    web_sys::console::time_with_label("filter-map-setup");
    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    web_sys::console::time_end_with_label("filter-map-setup");

    web_sys::console::time_with_label("filter-map-test");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    web_sys::console::time_end_with_label("filter-map-test");

    web_sys::console::time_with_label("filter-map-teardown");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
    web_sys::console::time_end_with_label("filter-map-teardown");

    web_sys::console::time_end_with_label("filter-map-suite");
}
