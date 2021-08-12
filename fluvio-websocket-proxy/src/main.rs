use async_std::prelude::*;
use tide_websockets::{Message, WebSocket};

use fluvio_future::net::{DefaultDomainConnector, TcpDomainConnector};
use tide::{Request, Result as TideResult};
use tide_websockets::WebSocketConnection;
use fluvio::config::ConfigFile;

use std::sync::Arc;
use std::sync::Mutex;
#[derive(Clone)]
struct State {
    disconnected: Arc<Mutex<bool>>,
}
impl std::default::Default for State {
    fn default() -> Self {
        Self {
            disconnected: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    /*
    let config_file = ConfigFile::default_config()?;
    let config = config_file.config();
    let cluster = config.current_cluster()?;
    */

    let state = State::default();
    let mut app = tide::with_state(state);
    app.at("/off").get(disconnect_all);
    app.at("/on").get(allow_connections);
    app.at("/")
        .with(WebSocket::new(ws_handler))
        .get(|_| async move { Ok("this was not a websocket request") });

    app.listen("127.0.0.1:3000").await?;

    Ok(())
}

async fn allow_connections(req: Request<State>) -> tide::Result {
    println!("Allowing connections!");
    let mut disconnected = req.state().disconnected.lock().unwrap();
    *disconnected = false;
    Ok(tide::Response::new(tide::StatusCode::Ok))
}

async fn disconnect_all(req: Request<State>) -> tide::Result {
    println!("DISCONNECTING ALL CONNECTIONS!");
    let mut disconnected = req.state().disconnected.lock().unwrap();
    *disconnected = true;
    Ok(tide::Response::new(tide::StatusCode::Ok))
}

#[derive(Debug, serde::Deserialize)]
struct WsQuery {
    domain: String,
}

async fn ws_handler(req: Request<State>, mut stream: WebSocketConnection) -> TideResult<()> {
    let domain = req.query::<WsQuery>().ok().map(|q| q.domain);
    let mut buf = vec![0; 10000];
    let connector = DefaultDomainConnector::new();

    // It would be cool if this looked at the domain
    let endpoint = if let Some(_domain) = domain {
        println!("CONNECTING TO DOMAIN: {:?}", _domain);
        "127.0.0.1:9010" // SPU
    } else {
        "127.0.0.1:9003" // SC
    };

    let (mut fluvio_writer, mut fluvio_reader, _fd) = connector.connect(&endpoint).await?;
    loop {
        tokio::select! {
            ws_next = stream.next() => {
                if *req.state().disconnected.lock().unwrap() {
                    break;
                }
                if let Some(Ok(Message::Binary(ws_in))) = ws_next {
                    let _ = fluvio_writer.write(&ws_in).await?;
                } else {
                    break;
                }
            }
            bytes_read = fluvio_reader.read(&mut buf) => {
                if *req.state().disconnected.lock().unwrap() {
                    break;
                }
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
