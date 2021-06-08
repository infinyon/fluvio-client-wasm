use async_trait::async_trait;
use fluvio_future::net::{
    BoxReadConnection, BoxWriteConnection, ConnectionFd, DomainConnector, TcpDomainConnector,
};
use fluvio_ws_stream_wasm::WsMeta;
use std::io::Error as IoError;
#[derive(Clone, Default)]
pub struct FluvioWebsocketConnector {
    url: String,
    domain: Option<String>,
}
impl FluvioWebsocketConnector {
    pub fn new(url: String, domain: Option<String>) -> Self {
        Self {
            url,
            domain,
        }
    }
}
#[async_trait(?Send)]
impl TcpDomainConnector for FluvioWebsocketConnector {
    async fn connect(
        &self,
        addr: &str,
    ) -> Result<(BoxWriteConnection, BoxReadConnection, ConnectionFd), IoError> {
        let url = if let Some(ref domain) = self.domain {
            format!("{}?domain={}", self.url, domain)
        } else {
            self.url.clone()
        };
        tracing::debug!("CONNECTING TO url: {:?}, passed in addr was {:?}", url, addr);

        let (mut _ws, wsstream) = WsMeta::connect(url.clone(), None)
            .await
            .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?;
        let wsstream_clone = wsstream.clone();
        Ok((
            Box::new(wsstream.into_io()),
            Box::new(wsstream_clone.into_io()),
            String::from(url),
        ))
    }

    fn new_domain(&self, domain: String) -> DomainConnector {
        Box::new(Self::new(self.url.clone(), Some(domain)))
    }

    fn domain(&self) -> &str {
        if let Some(domain) = &self.domain {
            &domain
        } else {
            ""
        }
    }
}
