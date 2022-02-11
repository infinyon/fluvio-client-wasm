use wasm_bindgen::prelude::*;

use fluvio::metadata::objects::Metadata;
use fluvio::metadata::tableformat::TableFormatSpec;

#[wasm_bindgen]
pub struct TableFormatMetadata {
    inner: Metadata<TableFormatSpec>,
}

#[wasm_bindgen]
impl TableFormatMetadata {
    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(method, getter = smartModule)]
    pub fn smartmodule(&self) -> Option<String> {
        self.inner.spec.smartmodule.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn columns(&self) -> Option<Vec<JsValue>> {
        self.inner.spec.columns.clone().map(|columns| {
            columns
                .into_iter()
                .map(|column| JsValue::from(TableFormatColumnConfigJs::from(column)))
                .collect()
        })
    }
}

impl From<Metadata<TableFormatSpec>> for TableFormatMetadata {
    fn from(inner: Metadata<TableFormatSpec>) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
pub struct TableFormatColumnConfigJs {
    inner: fluvio::metadata::tableformat::TableFormatColumnConfig,
}

#[wasm_bindgen]
impl TableFormatColumnConfigJs {
    #[wasm_bindgen(method, getter = headerLabel)]
    pub fn header_label(&self) -> Option<String> {
        self.inner.header_label.clone()
    }

    #[wasm_bindgen(method, getter = keyPath)]
    pub fn key_path(&self) -> String {
        self.inner.key_path.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn format(&self) -> Option<String> {
        self.inner.format.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn width(&self) -> Option<String> {
        self.inner.width.clone()
    }

    #[wasm_bindgen(method, getter = primaryKey)]
    pub fn primary_key(&self) -> bool {
        self.inner.primary_key.unwrap_or(false)
    }

    #[wasm_bindgen(method, getter)]
    pub fn display(&self) -> bool {
        self.inner.display.unwrap_or(false)
    }
}
impl From<fluvio::metadata::tableformat::TableFormatColumnConfig> for TableFormatColumnConfigJs {
    fn from(inner: fluvio::metadata::tableformat::TableFormatColumnConfig) -> Self {
        Self { inner }
    }
}
