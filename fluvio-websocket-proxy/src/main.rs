use async_std::prelude::*;
use tide_websockets::{Message, WebSocket};

use fluvio_future::net::{DefaultDomainConnector, TcpDomainConnector};
use tide::{Request, Result as TideResult};
use tide_websockets::WebSocketConnection;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let mut app = tide::new();
    app.at("/")
        .with(WebSocket::new(ws_handler))
        .get(|_| async move { Ok("this was not a websocket request") });

    app.listen("127.0.0.1:3000").await?;

    Ok(())
}
#[derive(Debug, serde::Deserialize)]
struct WsQuery {
    domain: String,
}

async fn ws_handler(req: Request<()>, mut stream: WebSocketConnection) -> TideResult<()> {
    let domain = req.query::<WsQuery>().ok().map(|q| q.domain);
    let mut buf = vec![0; 10000];
    let connector = DefaultDomainConnector::new();

    // It would be cool if this looked at the domain
    let endpoint = if let Some(_domain) = domain {
        "127.0.0.1:9010" // SC
    } else {
        "127.0.0.1:9003" // SC
    };

    let (mut fluvio_writer, mut fluvio_reader, _fd) = connector.connect(&endpoint).await?;
    loop {
        tokio::select! {
            ws_next = stream.next() => {
                if let Some(Ok(Message::Binary(ws_in))) = ws_next {
                    let _ = fluvio_writer.write(&ws_in).await?;
                } else {
                    break;
                }
            }
            bytes_read = fluvio_reader.read(&mut buf) => {
                if let Ok(bytes_read) = bytes_read {
                    let _ = stream.send_bytes(buf[..bytes_read].to_vec()).await;
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}
