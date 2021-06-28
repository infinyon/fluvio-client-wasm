use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use fluvio_client_wasm::*;

#[wasm_bindgen_test]
async fn simple() {
    let fluvio = Fluvio::connect("ws://localhost:3000".into()).await;
    assert!(fluvio.is_ok());
    let fluvio = fluvio.unwrap();
    let ret = simple_js(fluvio, Offset::from_end(1)).await;
    assert_eq!(ret.err(), None);
}

#[wasm_bindgen(module = "/tests/js/simple.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = simple)]
    async fn simple_js(fluvio: Fluvio, offset: Offset) -> Result<JsValue, JsValue>;
}
