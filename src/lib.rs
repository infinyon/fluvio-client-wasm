mod utils;

use tokio_stream::StreamExt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use fluvio::{
    config::FluvioConfig, consumer::Record as NativeRecord, Fluvio as NativeFluvio,
    FluvioError as NativeFluvioError, Offset as NativeOffset,
    PartitionConsumer as NativePartitionConsumer, TopicProducer as NativeTopicProducer,
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
impl From<String> for FluvioError {
    fn from(err: String) -> Self {
        Self {
            inner: NativeFluvioError::Other(err),
        }
    }
}

#[wasm_bindgen]
pub struct Offset {
    inner: NativeOffset,
}

impl From<NativeOffset> for Offset {
    fn from(inner: NativeOffset) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl Offset {
    pub fn from_beginning(offset: u32) -> Self {
        NativeOffset::from_beginning(offset).into()
    }
    pub fn beginning() -> Self {
        NativeOffset::beginning().into()
    }
    pub fn from_end(offset: u32) -> Self {
        NativeOffset::from_end(offset).into()
    }
    pub fn end() -> Self {
        NativeOffset::end().into()
    }
    /*
    pub fn absolute(index: i64) -> Result<Self, FluvioError>{
        NativeOffset::absolute(index).map(Offset::from).map_err(|e| FluvioError::from(e))
    }
    */
}

#[wasm_bindgen]
pub struct TopicProducer {
    inner: Rc<NativeTopicProducer>,
}
#[wasm_bindgen]
impl TopicProducer {
    pub fn send(&self, key: String, value: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.send(key, value).await
                .map(|_| JsValue::null()) //
                .map_err(|e| FluvioError::from(e).into())
        })
    }
}

impl From<NativeTopicProducer> for TopicProducer {
    fn from(inner: NativeTopicProducer) -> Self {
        Self { inner: Rc::new(inner) }
    }
}

#[wasm_bindgen]
pub struct Record {
    inner: NativeRecord,
}
#[wasm_bindgen]
impl Record {
    pub fn value(&self) -> Vec<u8> {
        self.inner.value().to_vec()
    }
    pub fn valueString(&self) -> Option<String> {
        String::from_utf8(self.inner.value().to_vec()).ok()
    }

    pub fn key(&self) -> Option<Vec<u8>> {
        self.inner.key().map(|v| v.to_vec())
    }
    pub fn keyString(&self) -> Option<String> {
        if let Some(key) = self.key() {
            String::from_utf8(key.to_vec()).ok()
        } else {
            None
        }
    }
    pub fn offset(&self) -> i64 {
        self.inner.offset()
    }
}


impl From<NativeRecord> for Record {
    fn from(inner: NativeRecord) -> Self {
        Self { inner }
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

#[wasm_bindgen]
pub struct Fluvio {
    inner: NativeFluvio,
}

#[wasm_bindgen]
impl Fluvio {
    pub async fn topic_producer(self, topic: String) -> Result<TopicProducer, FluvioError> {
        Ok(self.inner.topic_producer(topic).await?.into())
    }
    pub async fn partition_consumer(
        self,
        topic: String,
        partition: i32,
    ) -> Result<PartitionConsumer, FluvioError> {
        Ok(self
            .inner
            .partition_consumer(topic, partition)
            .await?
            .into())
    }
    pub async fn connect(addr: String) -> Result<Fluvio, FluvioError> {
        utils::set_panic_hook();
        let config = FluvioConfig::new(addr);
        let inner = NativeFluvio::connect_with_connector(
            Box::new(FluvioWebsocketConnector::new()),
            &config,
        ).await?;
        Ok(Self { inner })
    }
}

use wasm_client_connector::*;
mod wasm_client_connector {
    use async_trait::async_trait;
    use fluvio_ws_stream_wasm::WsMeta;
    use std::io::Error as IoError;
    use fluvio_future::{
        net::{
            BoxReadConnection,
            BoxWriteConnection,
            DomainConnector,
            TcpDomainConnector,
            ConnectionFd,
        },
    };
    #[derive(Clone, Default)]
    pub struct FluvioWebsocketConnector {}
    impl FluvioWebsocketConnector {
        pub fn new() -> Self {
            Self {}
        }
    }
    #[async_trait(?Send)]
    impl TcpDomainConnector for FluvioWebsocketConnector {
        async fn connect(
            &self,
            addr: &str,
        ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {

            let addr = if addr == "localhost:9010" {
                "ws://localhost:3001"
            } else {
                addr
            };

            let (mut _ws, wsstream) = WsMeta::connect(addr, None)
                .await
                .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?;
            let wsstream_clone = wsstream.clone();
            Ok((
                Box::new(wsstream.into_io()),
                Box::new(wsstream_clone.into_io()),
                String::from(addr),
            ))
        }

        fn new_domain(&self, _domain: String) -> DomainConnector {
            Box::new(self.clone())
        }

        fn domain(&self) -> &str {
            "localhost"
        }
    }
}



use tokio_stream::Stream;
//use wasm_bindgen_futures::stream::JsStream;
use std::rc::Rc;
use std::pin::Pin;
use std::cell::RefCell;

#[wasm_bindgen]
pub struct PartitionConsumerStream {
    inner: Rc<RefCell<Pin<Box<dyn Stream<Item = Result<Record, FluvioError>>>>>>,
}

use js_sys::Promise;
use js_sys::AsyncIterator;
use wasm_bindgen_futures::future_to_promise;

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

    pub fn async_iterator(self) -> AsyncIterator {
        AsyncIterator::from(JsValue::from(self))
    }
}
impl PartitionConsumerStream {
    pub async fn next_val(&self) -> Option<Result<Record, FluvioError>> {
        self.inner.borrow_mut().next().await
    }
}
