
//! Test suite for the Web and headless browsers.

use wasm_bindgen::prelude::*;
#[cfg(test)]
mod integration_tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    //use wasm_bindgen::convert::FromWasmAbi;
    //use wasm_bindgen::{JsCast, JsValue};

    #[wasm_bindgen_test]
    async fn js_integration_tests() {
        let ret = my_test().await;
        assert_eq!(ret.err(), None);
    }
}

#[wasm_bindgen(module = "/src/tests/js-tests/simple.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    //async fn my_test() -> Result<JsValue, JsValue>;
    async fn my_test() -> Result<JsValue, JsValue>;
}
