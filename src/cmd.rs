mod hdel;
mod hexist;
mod hget;
mod hset;

use prost::Message;

use crate::{
    pb::{command_request::RequestData, CommandRequest, CommandResponse, Kvpair, Value},
    value, Storage,
};
use anyhow::Result;
use bytes::{Bytes, BytesMut};
use http::StatusCode;

pub trait CommandExecutor {
    fn execute(self, storage: &dyn Storage) -> CommandResponse;
}

pub fn dispatch(request: CommandRequest, storage: &dyn Storage) -> CommandResponse {
    match request.request_data {
        Some(RequestData::Hget(param)) => param.execute(storage),
        Some(RequestData::Hset(param)) => param.execute(storage),
        Some(RequestData::Hdel(param)) => param.execute(storage),
        Some(RequestData::Hexist(param)) => param.execute(storage),
        Some(_) => todo!(),
        None => todo!(),
    }
}

impl CommandRequest {
    pub fn to_bytes(&self) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode(&mut buf)?;
        Ok(buf.into())
    }
}

impl CommandResponse {
    pub fn to_bytes(&self) -> Result<Bytes> {
        let mut buf = BytesMut::new();
        self.encode(&mut buf)?;
        Ok(buf.into())
    }
}

impl Value {
    pub fn to_bytes(&self) -> Result<Bytes> {
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

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self {
            value: Some(value::Value::Bool(v)),
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(v)),
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self {
            value: Some(value::Value::Float(v)),
        }
    }
}
