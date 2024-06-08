use bytes::BytesMut;
use kv_server::CommandRequest;
use prost::Message;

fn main() -> anyhow::Result<()> {
    let hset = CommandRequest::new_hset("table1", "hello", "world".into());
    let mut buf = BytesMut::new();
    hset.encode(&mut buf)?;
    // let hset2 = Message::decode(buf)?;
    let hset2 = CommandRequest::decode(buf)?;
    assert_eq!(hset, hset2);
    Ok(())
}
