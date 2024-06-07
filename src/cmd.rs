mod hdelete;
mod hget;
mod hset;
use prost::Message;

use crate::{
    error::KvError,
    pb::{command_request::RequestData, CommandRequest, CommandResponse, Kvpair, Value},
    value, Hset, Storage,
};
use anyhow::Result;
use bytes::{Bytes, BytesMut};
use http::StatusCode;

pub trait CommandExecutor {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(request: CommandRequest, store: &impl Storage) -> CommandResponse {
    match request.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        Some(RequestData::Hdel(param)) => param.execute(store),
        Some(_) => todo!(),
        None => todo!(),
    }
}

impl CommandRequest {
    pub fn new_hset(table: &str, key: &str, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn to_bytes(self) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode(&mut buf)?;
        Ok(buf.into())
    }
}

impl CommandResponse {
    pub fn to_bytes(self) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode(&mut buf)?;
        Ok(buf.into())
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

// From
impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Option<Value>> for CommandResponse {
    fn from(v: Option<Value>) -> Self {
        let v = match v {
            Some(v) => v,
            None => Value::default(),
        };

        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self {
            value: Some(value::Value::String(v)),
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self {
            value: Some(value::Value::String(v.to_string())),
        }
    }
}
