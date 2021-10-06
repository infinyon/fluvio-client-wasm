use async_std::prelude::*;
use tide_websockets::{Message, WebSocket};
use fluvio::config::{
    ConfigFile as FluvioConfigFile,
    FluvioConfig,
};

use fluvio_future::net::{DefaultDomainConnector, TcpDomainConnector};
use tide::{Request, Result as TideResult};
use tide_websockets::WebSocketConnection;



#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    addr: String,
}

async fn ws_handler(req: Request<()>, mut stream: WebSocketConnection) -> TideResult<()> {
    let domain = req.query::<WsQuery>().ok().map(|q| q.domain);
    let addr = req.query::<WsQuery>().ok().map(|q| q.addr);

    println!("NEW WS CONNECTION GOING TO {:?}, with domain {:?}", addr, domain);
    let endpoint = if let Some(addr) = addr {
        addr
    } else {
        let config_file = FluvioConfigFile::load_default_or_new().expect("Failed to load config file");
        let fluvio_config = config_file.config();
        let current_cluster = fluvio_config.current_cluster().expect("Failed to get current cluster");
        current_cluster.endpoint.clone()
    };


    let mut buf = vec![0; 10000];
    let connector = DefaultDomainConnector::new();


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
