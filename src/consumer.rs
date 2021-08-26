use fluvio::{
    ConsumerConfig as NativeConsumerConfig, PartitionConsumer as NativePartitionConsumer,
};
use js_sys::{Promise, Reflect};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::{FluvioError, Offset, Record};
use std::convert::{TryFrom, TryInto};

#[wasm_bindgen(typescript_custom_section)]
const CONSUMER_CONFIG_TYPE: &str = r#"
type SmartStreamFilter = {
    smartstreamType: "filter",
    smartstream: string,
}
type SmartStreamMap = {
    smartstreamType: "map",
    smartstream: string,
}
type SmartStreamAggregate = {
    smartstreamType: "aggregate",
    smartstream: string,
    accumulator: string | undefined,
}
type SmartStream = SmartStreamFilter | SmartStreamMap | SmartStreamAggregate | {};
type ConsumerConfig = SmartStream & {
    max_bytes: number | undefined,
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "SmartStream")]
    pub type SmartStream;
    #[wasm_bindgen(typescript_type = "ConsumerConfig")]
    pub type ConsumerConfig;
}

impl TryFrom<ConsumerConfig> for NativeConsumerConfig {
    type Error = String;

    fn try_from(js: ConsumerConfig) -> Result<Self, String> {
        let max_bytes = Reflect::get(&js, &"maxBytes".into())
            .ok()
            .and_then(|it| it.as_f64())
            .map(|it| it.round() as i32);

        let smartstream_type = Reflect::get(&js, &"smartstreamType".into())
            .ok()
            .and_then(|it| it.as_string());

        let smartstream_base64 = Reflect::get(&js, &"smartstream".into())
            .ok()
            .and_then(|it| it.as_string());

        let smartstream_accumulator = Reflect::get(&js, &"accumulator".into())
            .ok()
            .and_then(|it| it.as_string());

        // Builder for NativeConsumerConfig
        let mut builder = NativeConsumerConfig::builder();
        if let Some(max_bytes) = max_bytes {
            builder.max_bytes(max_bytes);
        }

        if let Some(wasm_base64) = smartstream_base64 {
            let wasm = base64::decode(wasm_base64)
                .map_err(|e| format!("Failed to decode SmartStream as a base64 string: {:?}", e))?;
            match smartstream_type.as_deref() {
                Some("filter") => {
                    builder.wasm_filter(wasm);
                }
                Some("map") => {
                    builder.wasm_map(wasm);
                }
                Some("aggregate") => {
                    let accumulator = smartstream_accumulator
                        .map(|acc| {
                            base64::decode(acc).map_err(|e| {
                                format!("Failed to decode Accumulator as a base64 string: {:?}", e)
                            })
                        })
                        .transpose()?
                        .unwrap_or_default();

                    builder.wasm_aggregate(wasm, accumulator);
                }
                _ => {
                    return Err(
                        "smartstreamType is required and must be 'filter', 'map', or 'aggregate'"
                            .to_string(),
                    )
                }
            }
        }
        let config = builder.build().map_err(|e| format!("{}", e))?;
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
