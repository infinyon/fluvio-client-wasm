use fluvio::{
    ConsumerConfig as NativeConsumerConfig, ConsumerConfigBuilder,
    PartitionConsumer as NativePartitionConsumer,
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

#[wasm_bindgen]
pub struct ConsumerConfig {
    inner: Rc<RefCell<Pin<Box<ConsumerConfigBuilder>>>>,
}

#[wasm_bindgen]
impl ConsumerConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Box::pin(NativeConsumerConfig::builder()))),
        }
    }

    #[wasm_bindgen(setter, js_name = "maxBytes")]
    pub fn set_max_bytes(&mut self, max_bytes: i32) {
        self.inner.borrow_mut().max_bytes(max_bytes);
    }

    #[wasm_bindgen(setter, js_name = "wasmFilterBinary")]
    pub fn set_wasm_filter_binary(&mut self, binary: Vec<u8>) {
        self.inner.borrow_mut().smartstream_binary(binary);
    }

    #[wasm_bindgen(setter, js_name = "wasmFilterBase64")]
    pub fn set_wasm_filter_base64(&mut self, base64: String) {
        self.inner.borrow_mut().smartstream_base64(base64).unwrap();
    }
}

#[wasm_bindgen]
pub struct PartitionConsumerStream {
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
        let config: NativeConsumerConfig = config.inner.borrow().build().unwrap();

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
