use async_trait::async_trait;
use fluvio_future::net::{
    BoxReadConnection, BoxWriteConnection, ConnectionFd, DomainConnector, TcpDomainConnector,
};
use fluvio_ws_stream_wasm::WsMeta;
use std::io::Error as IoError;
#[derive(Clone)]
pub struct FluvioWebsocketConnector {
    base_addr: String,
    token: String
}
impl FluvioWebsocketConnector {
    pub fn new(base_addr: String, token: String) -> Self {
        Self {
            base_addr,
            token,
        }
    }
}
#[async_trait(?Send)]
impl TcpDomainConnector for FluvioWebsocketConnector {
    async fn connect(
        &self,
        addr: &str,
    ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
        log::debug!("Connecting to {:?}", addr);
        let addr = if addr.ends_with("9005") {
            format!("{}{}", self.base_addr, "9005")
        } else if addr.ends_with("9003") {
            format!("{}{}", self.base_addr, "9003")
        } else {
            addr.to_string()
        }.to_string();
        log::debug!("Connecting to {:?}", addr);

        let (mut _ws, wsstream) = WsMeta::connect(addr.clone(), Some(vec![self.token.as_str()]))
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
