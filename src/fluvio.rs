use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use fluvio::{config::FluvioConfig, Fluvio as NativeFluvio};

use crate::{FluvioAdmin, FluvioError, FluvioWebsocketConnector, PartitionConsumer, TopicProducer};

#[wasm_bindgen]
pub struct Fluvio {
    inner: Rc<NativeFluvio>,
}

#[wasm_bindgen]
impl Fluvio {
    #[wasm_bindgen(js_name = topicProducer)]
    pub fn topic_producer(&self, topic: String) -> Promise {
        let rc = self.inner.clone();

        future_to_promise(async move {
            let topic_producer = rc
                .topic_producer(topic)
                .await
                .map(|producer| JsValue::from(TopicProducer::from(producer)))
                .map_err(|e| FluvioError::from(e).into());

            topic_producer
        })
    }
    #[wasm_bindgen(js_name = partitionConsumer)]
    pub fn partition_consumer(&self, topic: String, partition: i32) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let partition_consumer = rc
                .partition_consumer(topic, partition)
                .await
                .map(|consumer| JsValue::from(PartitionConsumer::from(consumer)))
                .map_err(|e| FluvioError::from(e).into());

            partition_consumer
        })
    }

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
    pub fn admin(&self) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let admin = JsValue::from(FluvioAdmin::from(rc.admin().await));

            Ok(admin)
        })
    }

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
