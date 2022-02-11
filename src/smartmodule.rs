use wasm_bindgen::prelude::*;

use fluvio::metadata::objects::Metadata;
use fluvio::metadata::smartmodule::SmartModuleSpec;

#[wasm_bindgen]
pub struct SmartModuleMetadata {
    inner: Metadata<SmartModuleSpec>,
}

#[wasm_bindgen]
impl SmartModuleMetadata {
    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }
}

impl From<Metadata<SmartModuleSpec>> for SmartModuleMetadata {
    fn from(inner: Metadata<SmartModuleSpec>) -> Self {
        Self { inner }
    }
}
