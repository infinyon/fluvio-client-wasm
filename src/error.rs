use fluvio::FluvioError as NativeFluvioError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct FluvioError {
    inner: NativeFluvioError,
}

// This is to get the stack for `FluvioError`
#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}
#[wasm_bindgen]
impl FluvioError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        format!("{:?}", self.inner)
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        "FluvioError".to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn stack(&self) -> String {
        let e = Error::new();
        e.stack()
    }
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
