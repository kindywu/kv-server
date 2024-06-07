use anyhow::Result;
use futures::SinkExt;
use kv_server::{CommandRequest, CommandResponse};
use prost::Message;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:8080"; // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    let request = CommandRequest::new_hset("table1", "hello", "world".into());
    let buf = request.to_bytes()?;

    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    framed.send(buf).await?;
    if let Some(Ok(data)) = framed.next().await {
        info!("Got response {:?}", CommandResponse::decode(data));
    }
    Ok(())
}
