use fluvio::consumer::Record as NativeRecord;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Record {
    inner: NativeRecord,
}
#[wasm_bindgen]
impl Record {
    pub fn value(&self) -> Vec<u8> {
        self.inner.value().to_vec()
    }
    #[wasm_bindgen(js_name = valueString)]
    pub fn value_string(&self) -> Option<String> {
        String::from_utf8(self.inner.value().to_vec()).ok()
    }

    pub fn key(&self) -> Option<Vec<u8>> {
        self.inner.key().map(|v| v.to_vec())
    }

    #[wasm_bindgen(js_name = keyString)]
    pub fn key_string(&self) -> Option<String> {
        if let Some(key) = self.key() {
            String::from_utf8(key.to_vec()).ok()
        } else {
            None
        }
    }
    pub fn offset(&self) -> i64 {
        self.inner.offset()
    }
}

impl From<NativeRecord> for Record {
    fn from(inner: NativeRecord) -> Self {
        Self { inner }
    }
}
