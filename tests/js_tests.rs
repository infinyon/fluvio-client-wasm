use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::*;

async fn get_fluvio() -> Fluvio {
    let fluvio = Fluvio::connect("ws://localhost:3000".into()).await;
    if let Err(e) = fluvio {
        let e = FluvioError::try_from(e);
        panic!("{:?}", e);
    }
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

    setup(get_fluvio().await)
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    test(get_fluvio().await, Offset::from_end(1))
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    teardown(get_fluvio().await)
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
}

#[wasm_bindgen_test]
async fn consumer_filter() {
    #[wasm_bindgen(module = "/tests/js/consumer_filter/consumer_filter.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn setup(fluvio: Fluvio) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn test(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn teardown(fluvio: Fluvio) -> Result<JsValue, JsValue>;
    }

    setup(get_fluvio().await)
        .await
        .map_err(FluvioError::try_from)
        .expect("Setup failed");
    test(get_fluvio().await, Offset::beginning())
        .await
        .map_err(FluvioError::try_from)
        .expect("Test failed");
    teardown(get_fluvio().await)
        .await
        .map_err(FluvioError::try_from)
        .expect("Teardown failed");
}
