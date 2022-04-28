use async_trait::async_trait;
use fluvio_future::net::{
    BoxReadConnection, BoxWriteConnection, ConnectionFd, DomainConnector, TcpDomainConnector,
};

use futures_util::io::AsyncReadExt;
use std::io::Error as IoError;
use ws_stream_wasm::WsMeta;
#[derive(Clone, Default)]
pub struct FluvioWebsocketConnector {
    url: String,
    domain: Option<String>,
}
impl FluvioWebsocketConnector {
    pub fn new(url: String, domain: Option<String>) -> Self {
        Self { url, domain }
    }
}
#[async_trait(?Send)]
impl TcpDomainConnector for FluvioWebsocketConnector {
    async fn connect(
        &self,
        addr: &str,
    ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
        let url = if let Some(ref domain) = self.domain {
            format!("{}?domain={}&addr={}", self.url, domain, addr)
        } else {
            self.url.clone()
        };
        tracing::debug!(
            "CONNECTING TO url: {:?}, passed in addr was {:?}, domain - {:?}",
            url,
            addr,
            self.domain
        );

        let (mut _ws, wsstream) = WsMeta::connect(url.clone(), None)
            .await
            .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?;

        let wsstream_io = wsstream.into_io();
        let (stream, sink) = wsstream_io.split();

        Ok((Box::new(sink), Box::new(stream), url))
    }

    fn new_domain(&self, domain: String) -> DomainConnector {
        Box::new(Self::new(self.url.clone(), Some(domain)))
    }

    fn domain(&self) -> &str {
        if let Some(domain) = &self.domain {
            domain
        } else {
            ""
        }
    }
}
