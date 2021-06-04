use std::cell::RefCell;
use std::rc::Rc;

use crate::topic::TopicMetadata;
use crate::FluvioError;
use fluvio::metadata::partition::PartitionSpec;
use fluvio::metadata::topic::TopicSpec;
use fluvio::FluvioAdmin as NativeFluvioAdmin;
use js_sys::Array;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub struct FluvioAdmin {
    inner: Rc<RefCell<NativeFluvioAdmin>>,
}
#[wasm_bindgen]
impl FluvioAdmin {
    #[wasm_bindgen(js_name = listTopics)]
    pub fn list_topics(&mut self) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let topic_list = rc
                .borrow_mut()
                .list::<TopicSpec, _>(vec![])
                .await
                .map(|topic_list| {
                    JsValue::from(
                        topic_list
                            .into_iter()
                            .map(|topic| JsValue::from(TopicMetadata::from(topic)))
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into());
            topic_list
        })
    }

    #[wasm_bindgen(js_name = listPartitions)]
    pub fn list_partitions(&mut self) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let partition_list = rc
                .borrow_mut()
                .list::<PartitionSpec, _>(vec![])
                .await
                .map(|partition_list| {
                    JsValue::from(
                        partition_list
                            .into_iter()
                            .map(|partition| JsValue::from(partition.name))
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into());
            partition_list
        })
    }
}

impl From<NativeFluvioAdmin> for FluvioAdmin {
    fn from(inner: NativeFluvioAdmin) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}
