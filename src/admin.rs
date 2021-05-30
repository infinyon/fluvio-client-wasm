use std::cell::RefCell;
use std::rc::Rc;
use std::pin::Pin;

use tokio_stream::Stream;
use js_sys::Array;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use fluvio::FluvioAdmin as NativeFluvioAdmin;
use fluvio::metadata::partition::PartitionSpec;
use fluvio::metadata::topic::TopicSpec;
use fluvio::metadata::objects::Metadata;
use fluvio::metadata::store::MetadataStoreObject;
use crate::topic::TopicMetadata;
use crate::FluvioError;

#[wasm_bindgen]
pub struct FluvioAdmin {
    inner: Rc<RefCell<NativeFluvioAdmin>>,
}

#[wasm_bindgen]
impl FluvioAdmin {
    pub fn list_topic(&mut self) -> Promise {
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

    pub fn list_partition(&mut self) -> Promise {
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

    pub fn watch_topics(&mut self) -> AsyncTopicStream {
        use tokio_stream::StreamExt;
        let stream = self.inner.borrow_mut().watch_topics().map(|it| {
            let (add, del) = it.parts();
            let convert = |meta: MetadataStoreObject<_, _>| TopicMetadata::from(Metadata {
                name: meta.key,
                spec: meta.spec,
                status: meta.status,
            });
            let added: Vec<_> = add.into_iter().map(convert).collect();
            let deleted: Vec<_> = del.into_iter().map(convert).collect();
            (added, deleted)
        });
        AsyncTopicStream {
            inner: Rc::new(RefCell::new(Box::pin(stream))),
        }
    }
}

impl From<NativeFluvioAdmin> for FluvioAdmin {
    fn from(inner: NativeFluvioAdmin) -> Self {
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}

macro_rules! impl_stream {
    ($stream:ident, $update:ident, $spec:ty) => {
        #[wasm_bindgen]
        pub struct $update {
            added: Vec<$spec>,
            deleted: Vec<$spec>,
        }

        #[wasm_bindgen]
        pub struct $stream {
            inner: Rc<RefCell<Pin<Box<dyn Stream<Item = (Vec<$spec>, Vec<$spec>)>>>>>,
        }

        #[wasm_bindgen]
        impl $stream {
            pub fn next(&mut self) -> Promise {
                use tokio_stream::StreamExt;

                let rc = self.inner.clone();
                future_to_promise(async move {
                    rc.borrow_mut().next().await
                        .ok_or_else(|| FluvioError::from(format!("{} watch stream closed", stringify!($spec))).into())
                        .map(|(added, deleted)| {
                            JsValue::from($update {
                                added,
                                deleted,
                            })
                        })
                })
            }
        }
    }
}

impl_stream!(AsyncTopicStream, TopicWatchUpdates, TopicMetadata);
// impl_stream!(AsyncPartitionStream, PartitionWatchUpdates, PartitionMetadata);
// impl_stream!(AsyncSpuStream, SpuWatchUpdates, SpuMetadata);
