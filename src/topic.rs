use std::rc::Rc;

use wasm_bindgen::prelude::*;

use fluvio::metadata::objects::Metadata;
use fluvio::metadata::topic::TopicSpec;

#[wasm_bindgen]
pub struct TopicMetadata {
    inner: Rc<Metadata<TopicSpec>>,
}

#[wasm_bindgen]
impl TopicMetadata {
    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn partitions(&self) -> i32 {
        self.inner.spec.partitions()
    }

    #[wasm_bindgen(method, getter = replicationFactor)]
    pub fn replication_factor(&self) -> Option<i32> {
        self.inner.spec.replication_factor()
    }
}

impl From<Metadata<TopicSpec>> for TopicMetadata {
    fn from(inner: Metadata<TopicSpec>) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}
