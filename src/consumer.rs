
use tokio_stream::StreamExt;
use fluvio::PartitionConsumer as NativePartitionConsumer;
use tokio_stream::Stream;
use std::rc::Rc;
use std::pin::Pin;
use std::cell::RefCell;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen::prelude::*;
use js_sys::Promise;

use crate::{
    Offset,
    FluvioError,
    Record,
};

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
                Ok(Some(val)) => {
                    Ok(val.into())
                },
                Ok(None) => {
                    Err(FluvioError::from("No value".to_string()).into())
                },
                Err(e) => {
                    Err(e.into())
                }
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
            inner: Rc::new(RefCell::new(Box::pin(self.inner.stream(offset.inner).await?.map(|result| {
                result
                    .map(|record| record.into())
                    .map_err(FluvioError::from)
            })))),
        })
    }
}
impl From<NativePartitionConsumer> for PartitionConsumer {
    fn from(inner: NativePartitionConsumer) -> Self {
        Self { inner }
    }
}
