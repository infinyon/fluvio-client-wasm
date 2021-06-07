use fluvio::metadata::partition::PartitionSpec;
use wasm_bindgen::prelude::*;

use fluvio::metadata::objects::Metadata;

#[wasm_bindgen]
pub struct PartitionMetadata {
    inner: Metadata<PartitionSpec>,
}

#[wasm_bindgen]
impl PartitionMetadata {
    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(method, getter = highWatermark)]
    pub fn high_watermark(&self) -> u32 {
        self.inner.status.leader.hw as u32
    }

    #[wasm_bindgen(method, getter = logEndOffset)]
    pub fn log_end_offset(&self) -> u32 {
        self.inner.status.leader.leo as u32
    }

    #[wasm_bindgen(method, getter = liveReplicas)]
    pub fn live_replicas(&self) -> u32 {
        self.inner.status.lsr()
    }
}

impl From<Metadata<PartitionSpec>> for PartitionMetadata {
    fn from(inner: Metadata<PartitionSpec>) -> Self {
        Self { inner }
    }
}
