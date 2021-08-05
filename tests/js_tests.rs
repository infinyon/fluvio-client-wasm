use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::*;

async fn get_fluvio() -> Fluvio {
    let fluvio = Fluvio::connect("ws://localhost:3000".into()).await;
    fluvio.expect("Failed to connect")
}

#[wasm_bindgen_test]
async fn simple() {
    let ret = simple::setup(get_fluvio().await).await;
    assert_eq!(ret.err(), None);
    let ret = simple::test(get_fluvio().await, Offset::from_end(1)).await;
    assert_eq!(ret.err(), None);
    let ret = simple::teardown(get_fluvio().await).await;
    assert_eq!(ret.err(), None);
}

#[wasm_bindgen_test]
async fn error_tests() {
    let ret = error_tests::test(get_fluvio().await, Offset::from_end(1)).await;
    assert_eq!(ret.err(), None);
}

mod simple {
    use super::*;
    #[wasm_bindgen(module = "/tests/js/simple.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup(fluvio: Fluvio) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown(fluvio: Fluvio) -> Result<JsValue, JsValue>;
    }
}
mod error_tests {
    use super::*;
    #[wasm_bindgen(module = "/tests/js/error_tests.js")]
    extern "C" {

        #[wasm_bindgen(catch)]
        pub async fn setup(fluvio: Fluvio) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown(fluvio: Fluvio) -> Result<JsValue, JsValue>;
    }
}
