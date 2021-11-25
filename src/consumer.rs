use fluvio::{
    ConsumerConfig as NativeConsumerConfig,
    MultiplePartitionConsumer as NativeMultiplePartitionConsumer,
    PartitionConsumer as NativePartitionConsumer,
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
export type SmartStreamType = "filter" | "map" | "aggregate";
export type ConsumerConfig = {
    max_bytes?: number,
    smartstreamType?: SmartStreamType,
    smartstream?: string,
    accumulator?: string,
    params?: object,
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
        use std::collections::BTreeMap;
        let params: BTreeMap<String, String> = Reflect::get(&js, &"params".into())
            .ok()
            .and_then(|it| it.into_serde().ok())
            .unwrap_or_default();

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
                    builder.wasm_filter(wasm, params);
                }
                Some("map") => {
                    builder.wasm_map(wasm, params);
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

                    builder.wasm_aggregate(wasm, accumulator, params);
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
use fluvio::dataplane::ErrorCode;

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
    pub async fn stream(
        self,
        offset: Offset,
    ) -> Result<PartitionConsumerStream, wasm_bindgen::JsValue> {
        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner
                    .stream(offset.inner)
                    .await
                    .map_err(FluvioError::from)?
                    .map(|result| {
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
    ) -> Result<PartitionConsumerStream, wasm_bindgen::JsValue> {
        let config: NativeConsumerConfig = config.try_into()?;

        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner
                    .stream_with_config(offset.inner, config)
                    .await
                    .map_err(FluvioError::from)?
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

#[wasm_bindgen]
pub struct MultiplePartitionConsumer {
    inner: NativeMultiplePartitionConsumer,
}

#[wasm_bindgen]
impl MultiplePartitionConsumer {
    pub async fn stream(
        self,
        offset: Offset,
    ) -> Result<PartitionConsumerStream, wasm_bindgen::JsValue> {
        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner
                    .stream(offset.inner)
                    .await
                    .map_err(FluvioError::from)?
                    .map(|result| {
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
    ) -> Result<PartitionConsumerStream, wasm_bindgen::JsValue> {
        let config: NativeConsumerConfig = config.try_into()?;

        Ok(PartitionConsumerStream {
            inner: Rc::new(RefCell::new(Box::pin(
                self.inner
                    .stream_with_config(offset.inner, config)
                    .await
                    .map_err(FluvioError::from)?
                    .map(|result| {
                        result
                            .map(|record| record.into())
                            .map_err(FluvioError::from)
                    }),
            ))),
        })
    }
}

impl From<NativeMultiplePartitionConsumer> for MultiplePartitionConsumer {
    fn from(inner: NativeMultiplePartitionConsumer) -> Self {
        Self { inner }
    }
}
