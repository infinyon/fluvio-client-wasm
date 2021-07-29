use fluvio::{
    ConsumerConfig as NativeConsumerConfig, PartitionConsumer as NativePartitionConsumer,
};
use js_sys::Promise;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{FluvioError, Offset, Record};
use std::convert::{TryFrom, TryInto};

#[wasm_bindgen]
pub struct ConsumerConfig {
    max_bytes: Option<i32>,
    smartstream_filter: Option<String>,
}

#[wasm_bindgen]
impl ConsumerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(js: JsValue) -> Self {
        let max_bytes = js_sys::Reflect::get(&js, &"maxBytes".into())
            .ok()
            .and_then(|it| it.as_f64())
            .map(|it| it.round() as i32);

        let smartstream_filter = js_sys::Reflect::get(&js, &"smartstreamFilter".into())
            .ok()
            .and_then(|it| it.as_string());

        Self {
            max_bytes,
            smartstream_filter,
        }
    }

    #[wasm_bindgen(setter, js_name = "maxBytes")]
    pub fn set_max_bytes(&mut self, max_bytes: i32) {
        self.max_bytes = Some(max_bytes);
    }

    #[wasm_bindgen(setter, js_name = "smartstreamFilter")]
    pub fn set_smartstream_filter(&mut self, string: String) {
        self.smartstream_filter = Some(string);
    }
}

impl TryFrom<ConsumerConfig> for NativeConsumerConfig {
    type Error = String;

    fn try_from(value: ConsumerConfig) -> Result<Self, Self::Error> {
        let mut config = NativeConsumerConfig::default();
        if let Some(max_bytes) = value.max_bytes {
            config = config.with_max_bytes(max_bytes);
        }
        if let Some(wasm_base64) = value.smartstream_filter {
            let wasm = base64::decode(wasm_base64)
                .map_err(|e| format!("Failed to decode SmartStream as a base64 string: {:?}", e))?;
            config = config.with_wasm_filter(wasm);
        }

        Ok(config)
    }
}

#[wasm_bindgen]
pub struct PartitionConsumerStream {
    #[allow(clippy::type_complexity)]
    inner: Rc<RefCell<Pin<Box<dyn Stream<Item = Result<Record, FluvioError>>>>>>,
}

#[wasm_bindgen]
impl PartitionConsumerStream {
    pub fn next(&self) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            match rc.borrow_mut().next().await.transpose() {
                Ok(Some(val)) => Ok(val.into()),
                Ok(None) => Err(FluvioError::from("No value".to_string()).into()),
                Err(e) => Err(e.into()),
            }
        })
    }
}
impl PartitionConsumerStream {
    pub async fn next_val(&self) -> Option<Result<Record, FluvioError>> {
        self.inner.borrow_mut().next().await
    }
}

#[wasm_bindgen]
pub struct PartitionConsumer {
    inner: NativePartitionConsumer,
}

#[wasm_bindgen]
impl PartitionConsumer {
    pub async fn stream(self, offset: Offset) -> Result<PartitionConsumerStream, FluvioError> {
        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner.stream(offset.inner).await?.map(|result| {
                    result
                        .map(|record| record.into())
                        .map_err(FluvioError::from)
                }),
            ))),
        })
    }

    #[wasm_bindgen(js_name = "streamWithConfig")]
    pub async fn stream_with_config(
        self,
        offset: Offset,
        config: ConsumerConfig,
    ) -> Result<PartitionConsumerStream, FluvioError> {
        let config: NativeConsumerConfig = config.try_into()?;

        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner
                    .stream_with_config(offset.inner, config)
                    .await?
                    .map(|result| {
                        result
                            .map(|record| record.into())
                            .map_err(FluvioError::from)
                    }),
            ))),
        })
    }
}

impl From<NativePartitionConsumer> for PartitionConsumer {
    fn from(inner: NativePartitionConsumer) -> Self {
        Self { inner }
    }
}
