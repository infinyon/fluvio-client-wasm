use wasm_bindgen::prelude::*;

use crate::{FluvioAdmin, FluvioError, FluvioWebsocketConnector, PartitionConsumer, TopicProducer};
use fluvio::{config::FluvioConfig, Fluvio as NativeFluvio};

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
        crate::utils::set_panic_hook();
        let config = FluvioConfig::new(addr);
        let inner = NativeFluvio::connect_with_connector(
            Box::new(FluvioWebsocketConnector::new()),
            &config,
        )
        .await?;
        Ok(Self { inner })
    }
    pub async fn admin(self) -> FluvioAdmin {
        self.inner.admin().await.into()
    }
}
