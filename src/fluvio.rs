use std::rc::Rc;

use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::future_to_promise;

use fluvio::{
    config::FluvioConfig, Compression, Fluvio as NativeFluvio, PartitionSelectionStrategy,
    TopicProducerConfig as NativeTopicProducerConfig,
    TopicProducerConfigBuilder as NativeTopicProducerConfigBuilder,
};

use crate::{
    FluvioAdmin, FluvioError, FluvioWebsocketConnector, MultiplePartitionConsumer,
    PartitionConsumer, TopicProducer,
};

use log::info;
use web_sys::console::log_1;

#[wasm_bindgen(typescript_custom_section)]
const PRODUCER_CONFIG_TYPE: &str = r#"
export type CompressionAlgorithm = "none" | "gzip" | "snappy" | "lz4";
export type TopicProducerConfig = {
    batchSize?: number,
    lingerTime?: number,
    compression?: CompressionAlgorithm,
}
"#;

impl TryFrom<TopicProducerConfig> for NativeTopicProducerConfig {
    type Error = String;

    fn try_from(config: TopicProducerConfig) -> Result<Self, Self::Error> {
        let mut builder = NativeTopicProducerConfigBuilder::default();
        let batch_size = Reflect::get(&config, &"batchSize".into())
            .ok()
            .and_then(|it| it.as_f64())
            .map(|it| it.round() as usize);

        if let Some(batch_size) = batch_size {
            builder = builder.batch_size(batch_size);
        }

        let linger_time = Reflect::get(&config, &"lingerTime".into())
            .ok()
            .and_then(|it| it.as_f64())
            .map(|it| std::time::Duration::from_millis(it.round() as u64));

        if let Some(linger_time) = linger_time {
            builder = builder.linger(linger_time);
        }

        let compression = match Reflect::get(&config, &"compression".into())
            .ok()
            .and_then(|it| it.as_string())
        {
            Some(compression) => Some(
                compression
                    .parse::<Compression>()
                    .map_err(|e| e.to_string())?,
            ),
            None => None,
        };

        if let Some(compression) = compression {
            builder = builder.compression(compression);
        }

        builder.build().map_err(|e| e.to_string())
    }
}

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

    #[wasm_bindgen(typescript_type = "CompressionAlgorithm")]
    pub type CompressionAlgorithm;
    #[wasm_bindgen(typescript_type = "TopicProducerConfig")]
    pub type TopicProducerConfig;

}

#[wasm_bindgen]
pub struct Fluvio {
    inner: Rc<NativeFluvio>,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum JsLevel {
    Error = "Error",
    Warn = "Warn",
    Info = "Info",
    Debug = "Debug",
    Trace = "Trace"
}

use log::Level;

impl Into<Level> for JsLevel {
    fn into(self) -> Level {
        match self {
            Self::Error => Level::Error,
            Self::Warn => Level::Warn,
            Self::Info => Level::Info,
            Self::Debug => Level::Debug,
            Self::Trace => Level::Trace,
            _ => Level::Error
        }
    }
}

#[wasm_bindgen]
impl Fluvio {
    /// Creates a new topic producer.
    #[wasm_bindgen(js_name = topicProducer)]
    pub fn topic_producer(&self, topic: String) -> PromiseTopicProducer {
        let rc = self.inner.clone();

        let promise = future_to_promise(async move {
            let result = rc.topic_producer(&topic)
                .await
                .map(|producer| JsValue::from(TopicProducer::from(producer)))
                .map_err(|e| (FluvioError::from(e).into()));

            info!("Produced topic: {:#?}", topic);

            result
        });

        use web_sys::console::log_1;
        log_1(&"Connected to fluvio server".into());

        // WARNING: this does not validate the return type. Check carefully.
        promise.unchecked_into::<PromiseTopicProducer>()
    }

    #[wasm_bindgen(js_name = topicProducerWithConfig)]
    pub fn topic_producer_with_config(
        &self,
        topic: String,
        config: TopicProducerConfig,
    ) -> PromiseTopicProducer {
        let rc = self.inner.clone();

        let promise = future_to_promise(async move {
            let config: NativeTopicProducerConfig = config.try_into()?;

            rc.topic_producer_with_config(topic, config)
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
        //Self::setup_debugging(false, JsLevel::Debug);
        console_error_panic_hook::set_once();

        let config = FluvioConfig::new(addr.clone());

        let inner = Rc::new(
            NativeFluvio::connect_with_connector(
                Box::new(FluvioWebsocketConnector::new(addr, None)),
                &config,
            )
            .await
            .map_err(FluvioError::from)?,
        );

        use log::info;
        info!("Connected to fluvio server");

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
    pub fn setup_debugging(enable_tracing_wasm: bool, level: JsLevel) {
        console_error_panic_hook::set_once();

        let msg = format!("Debug level: {:#?}", level);
        log_1(&msg.into());

        use std::sync::Once;
        static START: Once = Once::new();
        START.call_once(|| {
            if enable_tracing_wasm {
                tracing_wasm::set_as_global_default();
            }

            let level: Level = match level {
                JsLevel::Error => Level::Error,
                JsLevel::Warn => Level::Warn,
                JsLevel::Info => Level::Info,
                JsLevel::Debug => Level::Debug,
                JsLevel::Trace => Level::Trace,
                _ => {
                    return;
                }
            };

            let msg = format!("Effective debug level: {:#?}", level);
            log_1(&msg.into());

            console_log::init_with_level(level).expect("error initializing log");
        });
    }
}
