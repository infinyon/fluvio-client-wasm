use fluvio::FluvioAdmin as NativeFluvioAdmin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FluvioAdmin {
    inner: NativeFluvioAdmin,
}

impl From<NativeFluvioAdmin> for FluvioAdmin {
    fn from(inner: NativeFluvioAdmin) -> Self {
        Self { inner }
    }
}
