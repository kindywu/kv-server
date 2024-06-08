use std::{env, net::SocketAddr, path::Path, sync::Arc};

use anyhow::Result;
use dotenv::dotenv;
use futures::{SinkExt, StreamExt};
use kv_server::{dispatch, CommandRequest, MemoryStorage, RedbStorage, Storage};
use prost::Message;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let listen_addr = env::var("LISTEN_ADDR")?;
    let storage_type = env::var("STORAGE_TYPE")?;

    info!("kv server listen on {listen_addr}");
    let listener = TcpListener::bind(listen_addr).await?;

    let storage: Arc<dyn Storage> = match storage_type.as_str() {
        "memory" => Arc::new(MemoryStorage::new()),
        "redb" => {
            let file = env::var("STORAGE_REDB_FILE")?;
            let file = Path::new(&file);
            Arc::new(RedbStorage::try_new(file)?)
        }
        _ => panic!(""),
    };

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("kv server accept client {addr}");

        let storage = storage.clone();
        tokio::spawn(async move {
            match handle(stream, addr, storage).await {
                Ok(_) => info!("client {} leave", addr),
                Err(e) => warn!("client {} leave with error {:?}", addr, e),
            }
        });
    }
}

async fn handle(stream: TcpStream, addr: SocketAddr, storage: Arc<dyn Storage>) -> Result<()> {
    info!("kv server handle request from {}", addr);
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    let storage = storage.as_ref();
    loop {
        match framed.next().await {
            Some(Ok(buf)) => {
                let request = CommandRequest::decode(buf)?;
                let response = dispatch(request, storage);

                let buf = response.to_bytes()?;

                framed.send(buf).await?;
            }
            Some(Err(e)) => {
                warn!("receive from {} with error {:?}", addr, e);
                break;
            }
            None => break,
        }
    }
    Ok(())
}
