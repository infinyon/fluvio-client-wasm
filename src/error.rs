use fluvio::FluvioError as NativeFluvioError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct FluvioError {
    inner: FluvioInner,
}

#[derive(Debug)]
pub enum FluvioInner {
    Fluvio(NativeFluvioError),
    Code(fluvio::dataplane::ErrorCode),
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
        Self {
            inner: FluvioInner::Fluvio(inner),
        }
    }
}
use fluvio::dataplane::ErrorCode as NativeErrorCode;

impl From<NativeErrorCode> for FluvioError {
    fn from(inner: NativeErrorCode) -> Self {
        Self {
            inner: FluvioInner::Code(inner),
        }
    }
}
impl From<String> for FluvioError {
    fn from(err: String) -> Self {
        Self {
            inner: FluvioInner::Fluvio(NativeFluvioError::Other(err)),
        }
    }
}

use std::convert::TryFrom;
impl TryFrom<JsValue> for FluvioError {
    type Error = JsValue;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        crate::generic_of_jsval(value, "FluvioError")
    }
}
