mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use fluvio::{
    Fluvio as NativeFluvio,
    FluvioError as NativeFluvioError,
    TopicProducer as NativeTopicProducer,
    config::FluvioConfig,
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct FluvioError {
    inner: NativeFluvioError,
}
impl From<NativeFluvioError> for FluvioError {
    fn from(inner: NativeFluvioError) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
pub struct TopicProducer {
    _inner: NativeTopicProducer,
}
impl From<NativeTopicProducer> for TopicProducer {
    fn from(_inner: NativeTopicProducer) -> Self {
        Self { _inner }
    }
}

#[wasm_bindgen]
pub struct Fluvio {
    _inner: NativeFluvio,
}

#[wasm_bindgen]
impl Fluvio {
    pub async fn connect() -> Result<Fluvio, FluvioError> {
        utils::set_panic_hook();
        let config = FluvioConfig::new("ws://localhost:8080");
        let _inner = NativeFluvio::connect_with_config(&config).await?;
        Ok(Self {
            _inner
        })
    }
    /*
    pub async fn topic_producer(&self, topic: String) -> Result<TopicProducer, FluvioError> {
        Ok(self.inner.topic_producer(topic).await?.into())
    }
    */
}
