use async_trait::async_trait;
use fluvio_future::net::{
    BoxReadConnection, BoxWriteConnection, ConnectionFd, DomainConnector, TcpDomainConnector,
};
use fluvio_ws_stream_wasm::WsMeta;
use std::io::Error as IoError;
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
