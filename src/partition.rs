use std::rc::Rc;

use wasm_bindgen::prelude::*;

use fluvio::metadata::objects::Metadata;
use fluvio::metadata::partition::PartitionSpec;

#[wasm_bindgen]
pub struct PartitionMetadata {
    inner: Rc<Metadata<PartitionSpec>>,
}

#[wasm_bindgen]
impl PartitionMetadata {

    #[wasm_bindgen(method, getter)]
        pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(method, getter = highWatermark)]
    pub fn high_watermark(&self) -> i64 {
        self.inner.status.leader.hw
    }

    #[wasm_bindgen(method, getter = logEndOffset)]
    pub fn log_end_offset(&self) -> i64 {
        self.inner.status.leader.leo
    }
}

impl From<Metadata<PartitionSpec>> for PartitionMetadata {
    fn from(inner: Metadata<PartitionSpec>) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}
