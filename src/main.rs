use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use kv_server::{dispatch, CommandRequest, MemoryStorage, Storage};
use prost::Message;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:8080"; // 连接服务器
    let listener = TcpListener::bind(addr).await?;

    let storage = Arc::new(MemoryStorage::new(DashMap::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let storage = storage.clone();
        tokio::spawn(async move {
            match handle(stream, addr, storage).await {
                Ok(_) => info!("{} quit", addr),
                Err(e) => warn!("{} quit with error {:?}", addr, e),
            }
        });
    }
}

async fn handle(stream: TcpStream, addr: SocketAddr, storage: Arc<impl Storage>) -> Result<()> {
    info!("handle request from {}", addr);
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
    loop {
        match framed.next().await {
            Some(Ok(buf)) => {
                let request = CommandRequest::decode(buf)?;
                let response = dispatch(request, storage.as_ref());

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
