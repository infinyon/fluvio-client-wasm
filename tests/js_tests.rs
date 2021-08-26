use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::*;

async fn get_fluvio() -> Fluvio {
    let fluvio = Fluvio::connect("ws://localhost:3000".into()).await;
    assert!(fluvio.is_ok());
    fluvio.unwrap()
}

#[wasm_bindgen_test]
async fn simple() {
    #[wasm_bindgen(module = "/tests/js/simple/simple.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup(fluvio: Fluvio) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown(fluvio: Fluvio) -> Result<JsValue, JsValue>;
    }

    setup(get_fluvio().await).await.unwrap();
    test(get_fluvio().await, Offset::from_end(1)).await.unwrap();
    teardown(get_fluvio().await).await.unwrap();
}

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

    setup(get_fluvio().await).await.unwrap();
    test(get_fluvio().await, Offset::from_end(1)).await.unwrap();
    teardown(get_fluvio().await).await.unwrap();
}
