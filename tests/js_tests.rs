use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::FluvioError;

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

    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
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

    setup()
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    test()
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    teardown()
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
}

/*
#[wasm_bindgen_test]
async fn smartstream_filter() {
    #[wasm_bindgen(module = "/tests/js/smartstream_filter/smartstream_filter.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup(fluvio: Fluvio) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown(fluvio: Fluvio) -> Result<JsValue, JsValue>;
    }

    setup(get_fluvio().await).await.map_err(FluvioError::try_from).expect("Setup failed");
    test(get_fluvio().await, Offset::from_end(1)).await.map_err(FluvioError::try_from).expect("Test failed");
    teardown(get_fluvio().await).await.map_err(FluvioError::try_from).expect("Teardown failed");
}
*/
