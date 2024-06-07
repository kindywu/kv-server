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

    let mut framed: Framed<TcpStream, LengthDelimitedCodec> =
        Framed::new(stream, LengthDelimitedCodec::new());

    // hset
    let hset = CommandRequest::new_hset("table1", "hello", "world".into());
    let buf = hset.to_bytes()?;

    framed.send(buf).await?;
    if let Some(Ok(data)) = framed.next().await {
        info!("send hset got response {:?}", CommandResponse::decode(data));
    }

    // hexist
    let hexist = CommandRequest::new_hexist("table1", "hello");
    let buf = hexist.to_bytes()?;

    framed.send(buf).await?;
    if let Some(Ok(data)) = framed.next().await {
        info!(
            "send hexist got response {:?}",
            CommandResponse::decode(data)
        );
    }

    // hget
    let hget = CommandRequest::new_hget("table1", "hello");
    let buf = hget.to_bytes()?;

    framed.send(buf).await?;
    if let Some(Ok(data)) = framed.next().await {
        info!("send hget got response {:?}", CommandResponse::decode(data));
    }

    // hdel
    let hdel = CommandRequest::new_hdel("table1", "hello");
    let buf = hdel.to_bytes()?;

    framed.send(buf).await?;
    if let Some(Ok(data)) = framed.next().await {
        info!("send hdel got response {:?}", CommandResponse::decode(data));
    }
    Ok(())
}
