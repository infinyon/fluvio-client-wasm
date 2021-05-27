use fluvio::FluvioError as NativeFluvioError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct FluvioError {
    inner: NativeFluvioError,
}

impl From<NativeFluvioError> for FluvioError {
    fn from(inner: NativeFluvioError) -> Self {
        Self { inner }
    }
}
impl From<String> for FluvioError {
    fn from(err: String) -> Self {
        Self {
            inner: NativeFluvioError::Other(err),
        }
    }
}
