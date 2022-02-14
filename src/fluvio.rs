use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

use fluvio::{config::FluvioConfig, Fluvio as NativeFluvio, PartitionSelectionStrategy};

use crate::{
    FluvioAdmin, FluvioError, FluvioWebsocketConnector, MultiplePartitionConsumer,
    PartitionConsumer, TopicProducer,
};

// Workaround for Typescript type annotations on async function returns.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Promise<TopicProducer>")]
    pub type PromiseTopicProducer;

    #[wasm_bindgen(typescript_type = "Promise<PartitionConsumer>")]
    pub type PromisePartitionConsumer;

    #[wasm_bindgen(typescript_type = "Promise<MultiplePartitionConsumer>")]
    pub type PromiseMultiplePartitionConsumer;

    #[wasm_bindgen(typescript_type = "Promise<FluvioAdmin>")]
    pub type PromiseFluvioAdmin;
}

#[wasm_bindgen]
pub struct Fluvio {
    inner: Rc<NativeFluvio>,
}

#[wasm_bindgen]
impl Fluvio {
    /// Creates a new topic producer.
    #[wasm_bindgen(js_name = topicProducer)]
    pub fn topic_producer(&self, topic: String) -> PromiseTopicProducer {
        let rc = self.inner.clone();

        let promise = future_to_promise(async move {
            rc.topic_producer(topic)
                .await
                .map(|producer| JsValue::from(TopicProducer::from(producer)))
                .map_err(|e| (FluvioError::from(e).into()))
        });

        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseTopicProducer>()
    }

    /// Creates a new partition consumer
    #[wasm_bindgen(js_name = partitionConsumer)]
    pub fn partition_consumer(&self, topic: String, partition: i32) -> PromisePartitionConsumer {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.partition_consumer(topic, partition)
                .await
                .map(|consumer| JsValue::from(PartitionConsumer::from(consumer)))
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromisePartitionConsumer>()
    }

    /// Creates a multiple partition consumer
    #[wasm_bindgen(js_name = allPartitionsConsumer)]
    pub fn all_partitions_consumer(&self, topic: String) -> PromiseMultiplePartitionConsumer {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            rc.consumer(PartitionSelectionStrategy::All(topic))
                .await
                .map(|consumer| JsValue::from(MultiplePartitionConsumer::from(consumer)))
                .map_err(|e| FluvioError::from(e).into())
        });
        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseMultiplePartitionConsumer>()
    }

    /// Connects to fluvio server
    pub async fn connect(addr: String) -> Result<Fluvio, wasm_bindgen::JsValue> {
        Self::setup_debugging(false);

        let config = FluvioConfig::new(addr.clone());

        let inner = Rc::new(
            NativeFluvio::connect_with_connector(
                Box::new(FluvioWebsocketConnector::new(addr, None)),
                &config,
            )
            .await
            .map_err(FluvioError::from)?,
        );
        Ok(Self { inner })
    }

    /// Creates fluvio admin instance
    pub fn admin(&self) -> PromiseFluvioAdmin {
        let rc = self.inner.clone();
        let promise = future_to_promise(async move {
            let admin = JsValue::from(FluvioAdmin::from(rc.admin().await));

            Ok(admin)
        });
        promise.unchecked_into::<PromiseFluvioAdmin>()
    }

    /// enable debug logging
    #[wasm_bindgen(js_name = setupDebugging)]
    pub fn setup_debugging(verbose_debugging: bool) {
        console_error_panic_hook::set_once();
        if verbose_debugging {
            use std::sync::Once;
            static START: Once = Once::new();
            START.call_once(|| {
                tracing_wasm::set_as_global_default();
                use log::Level;
                console_log::init_with_level(Level::Debug).expect("error initializing log");
            });
        }
    }
}
