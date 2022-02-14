use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use fluvio::metadata::smartmodule::SmartModuleSpec;
use fluvio::metadata::smartmodule::SmartModuleWasm;
use js_sys::Array;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use fluvio::metadata::connector::ManagedConnectorSpec;
use fluvio::metadata::partition::PartitionSpec;
use fluvio::metadata::tableformat::TableFormatSpec;
use fluvio::metadata::topic::TopicSpec;
use fluvio::FluvioAdmin as NativeFluvioAdmin;

use crate::partition::PartitionMetadata;
use crate::smartmodule::SmartModuleMetadata;
use crate::tableformat::TableFormatMetadata;
use crate::topic::TopicMetadata;
use crate::FluvioError;

#[cfg(feature = "unstable")]
use fluvio::metadata::{objects::Metadata, store::MetadataStoreObject};

// Workaround for Typescript type annotations on async function returns.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Promise<TopicMetadata[]>")]
    pub type PromiseTopicList;

    #[wasm_bindgen(typescript_type = "Promise<PartitionMetadata[]>")]
    pub type PromisePartitionList;

    #[wasm_bindgen(typescript_type = "Promise<string[]>")]
    pub type PromiseConnectorList;

    #[wasm_bindgen(typescript_type = "Promise<SmartModuleMetadata[]>")]
    pub type PromiseSmartModuleList;

    #[wasm_bindgen(typescript_type = "Promise<TableFormatMetadata[]>")]
    pub type PromiseTableFormatList;
}

#[wasm_bindgen]
pub struct FluvioAdmin {
    inner: Rc<NativeFluvioAdmin>,
}
#[wasm_bindgen]
impl FluvioAdmin {
    /// List topics
    #[wasm_bindgen(js_name = listTopics)]
    pub fn list_topics(&mut self) -> PromiseTopicList {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.list::<TopicSpec, _>(vec![])
                .await
                .map(|topic_list| {
                    JsValue::from(
                        topic_list
                            .into_iter()
                            .map(|topic| JsValue::from(TopicMetadata::from(topic)))
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseTopicList>()
    }

    /// Create a new topic with the given name and partitions
    #[wasm_bindgen(js_name = createTopic)]
    pub fn create_topic(&self, topic_name: String, partition: i32) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.create(
                topic_name.clone(),
                false,
                TopicSpec::new_computed(partition, 1, Some(false)),
            )
            .await
            .map(|_| JsValue::from(topic_name))
            .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// Delete a topic
    #[wasm_bindgen(js_name = deleteTopic)]
    pub fn delete_topic(&self, topic_name: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.delete::<TopicSpec, String>(topic_name)
                .await
                .map(|_| JsValue::NULL)
                .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// List all partitions
    #[wasm_bindgen(js_name = listPartitions)]
    pub fn list_partitions(&mut self) -> PromisePartitionList {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.list::<PartitionSpec, _>(vec![])
                .await
                .map(|partition_list| {
                    JsValue::from(
                        partition_list
                            .into_iter()
                            .map(|partition| JsValue::from(PartitionMetadata::from(partition)))
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromisePartitionList>()
    }

    /// Watch topic updates
    #[wasm_bindgen(js_name = watchTopics)]
    pub fn watch_topics(&mut self) -> AsyncTopicStream {
        use tokio_stream::StreamExt;
        let stream = self.inner.watch_topics().map(|it| {
            let (add, del) = it.parts();
            let convert = |meta: MetadataStoreObject<_, _>| {
                TopicMetadata::from(Metadata {
                    name: meta.key,
                    spec: meta.spec,
                    status: meta.status,
                })
            };
            let added: Vec<_> = add.into_iter().map(convert).collect();
            let deleted: Vec<_> = del.into_iter().map(convert).collect();
            (added, deleted)
        });
        AsyncTopicStream {
            inner: Rc::new(RefCell::new(Box::pin(stream))),
        }
    }

    /// Watch partition updates
    #[wasm_bindgen(js_name = watchPartitions)]
    pub fn watch_partitions(&mut self) -> AsyncPartitionStream {
        use tokio_stream::StreamExt;
        let stream = self.inner.watch_partitions().map(|it| {
            let (add, del) = it.parts();
            let convert = |meta: MetadataStoreObject<PartitionSpec, _>| {
                PartitionMetadata::from(Metadata {
                    name: meta.key.to_string(),
                    spec: meta.spec,
                    status: meta.status,
                })
            };
            let added: Vec<_> = add.into_iter().map(convert).collect();
            let deleted: Vec<_> = del.into_iter().map(convert).collect();
            (added, deleted)
        });
        AsyncPartitionStream {
            inner: Rc::new(RefCell::new(Box::pin(stream))),
        }
    }

    //// Create a managed connector with the given configurations
    #[wasm_bindgen(js_name = createConnector)]
    pub fn create_connector(
        &self,
        name: String,
        type_: String,
        topic: String,
        parameters: &JsValue,
        secrets: &JsValue,
    ) -> Promise {
        use fluvio::metadata::connector::SecretString;
        let parameters: BTreeMap<String, String> = parameters.into_serde().unwrap_or_else(|e| {
            log::error!("Failed to get parameters from js {:?}", e);
            BTreeMap::new()
        });
        let secrets: BTreeMap<String, SecretString> = secrets
            .into_serde()
            .unwrap_or_else(|e| {
                log::error!("Failed to get parameters from js {:?}", e);
                BTreeMap::<String, String>::new()
            })
            .into_iter()
            .map(|(key, value)| (key, SecretString::from(value)))
            .collect();
        log::debug!("PARAMETERS {:?}", parameters);
        log::debug!("secrets {:?}", secrets);
        let connector_spec: ManagedConnectorSpec = ManagedConnectorSpec {
            name: name.clone(),
            type_,
            topic,
            parameters,
            secrets,
            ..Default::default()
        };
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.create(name.clone(), false, connector_spec)
                .await
                .map(|_| JsValue::from(name))
                .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// List connectors
    #[wasm_bindgen(js_name = listConnectors)]
    pub fn list_connectors(&mut self) -> PromiseConnectorList {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.list::<ManagedConnectorSpec, _>(vec![])
                .await
                .map(|topic_list| {
                    JsValue::from(
                        topic_list
                            .into_iter()
                            .map(|connector| JsValue::from(connector.name))
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into()
    }

    /// Delete a connector
    #[wasm_bindgen(js_name = deleteConnector)]
    pub fn delete_connector(&self, connector_name: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.delete::<ManagedConnectorSpec, String>(connector_name)
                .await
                .map(|_| JsValue::NULL)
                .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// Create a smartmodule
    #[wasm_bindgen(js_name = createSmartModule)]
    pub fn create_smartmodule(&self, name: String, wasm_body_base64: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let wasm = base64::decode(wasm_body_base64)
                .map_err(|e| format!("Failed to decode SmartModule as a base64 string: {:?}", e))?;

            let smartmodule_spec: SmartModuleSpec = SmartModuleSpec {
                wasm: SmartModuleWasm::from_binary_payload(wasm),
                ..Default::default()
            };
            rc.create(name.clone(), false, smartmodule_spec)
                .await
                .map(|_| JsValue::from(name))
                .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// List smartmodules
    #[wasm_bindgen(js_name = listSmartModule)]
    pub fn list_smartmodules(&mut self) -> PromiseSmartModuleList {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.list::<SmartModuleSpec, _>(vec![])
                .await
                .map(|smartmodule_list| {
                    JsValue::from(
                        smartmodule_list
                            .into_iter()
                            .map(|smartmodule| {
                                JsValue::from(SmartModuleMetadata::from(smartmodule))
                            })
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseSmartModuleList>()
    }

    /// Delete a smartmodule
    #[wasm_bindgen(js_name = deleteSmartModule)]
    pub fn delete_smartmodule(&self, sm_name: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.delete::<SmartModuleSpec, String>(sm_name)
                .await
                .map(|_| JsValue::NULL)
                .map_err(|e| FluvioError::from(e).into())
        })
    }

    /// List table formats
    #[wasm_bindgen(js_name = listTableFormat)]
    pub fn list_table_format(&mut self) -> PromiseTableFormatList {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.list::<TableFormatSpec, _>(vec![])
                .await
                .map(|table_format_list| {
                    JsValue::from(
                        table_format_list
                            .into_iter()
                            .map(|table_format| {
                                JsValue::from(TableFormatMetadata::from(table_format))
                            })
                            .collect::<Array>(),
                    )
                })
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseTableFormatList>()
    }

    /// Delete a table format
    #[wasm_bindgen(js_name = deleteTableFormat)]
    pub fn delete_table_format(&self, tf_name: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.delete::<TableFormatSpec, String>(tf_name)
                .await
                .map(|_| JsValue::NULL)
                .map_err(|e| FluvioError::from(e).into())
        })
    }
}

impl From<NativeFluvioAdmin> for FluvioAdmin {
    fn from(inner: NativeFluvioAdmin) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

macro_rules! impl_stream {
    ($stream:ident, $update:ident, $spec:ty) => {
        #[wasm_bindgen]
        pub struct $update {
            #[allow(dead_code)]
            added: Vec<$spec>,
            #[allow(dead_code)]
            deleted: Vec<$spec>,
        }

        #[wasm_bindgen]
        pub struct $stream {
            inner: Rc<
                RefCell<
                    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = (Vec<$spec>, Vec<$spec>)>>>,
                >,
            >,
        }

        #[wasm_bindgen]
        impl $stream {
            pub fn next(&mut self) -> Promise {
                use tokio_stream::StreamExt;

                let rc = self.inner.clone();
                future_to_promise(async move {
                    rc.borrow_mut()
                        .next()
                        .await
                        .ok_or_else(|| {
                            FluvioError::from(format!("{} watch stream closed", stringify!($spec)))
                                .into()
                        })
                        .map(|(added, deleted)| JsValue::from($update { added, deleted }))
                })
            }
        }
    };
}

impl_stream!(AsyncTopicStream, TopicWatchUpdates, TopicMetadata);
impl_stream!(
    AsyncPartitionStream,
    PartitionWatchUpdates,
    PartitionMetadata
);
