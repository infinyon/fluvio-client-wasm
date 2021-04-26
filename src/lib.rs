mod utils;

use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, fluvio-client-wasm!");
}

use fluvio::{
    Fluvio as FluvioNative,
    FluvioError as FluvioNativeError,
};


#[wasm_bindgen]
pub struct FluvioError {
    inner: FluvioNativeError,
}
impl From<FluvioNativeError> for FluvioError {
    fn from(inner: FluvioNativeError) -> Self {
        Self {
            inner
        }
    }
}

#[wasm_bindgen]
pub struct Fluvio {
    inner: FluvioNative,
}

#[wasm_bindgen]
impl Fluvio {
    pub async fn connect() -> Result<Fluvio, FluvioError> {
        utils::set_panic_hook();
        Ok(Self {inner: FluvioNative::connect().await?})
    }
}
