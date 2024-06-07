mod cmd;
mod error;
mod pb;
mod storage;

pub use cmd::*;
use command_request::RequestData;
pub use pb::*;
pub use storage::*;

// for client test
impl CommandRequest {
    pub fn new_hget(table: &str, key: &str) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    pub fn new_hset(table: &str, key: &str, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
    pub fn new_hdel(table: &str, key: &str) -> Self {
        Self {
            request_data: Some(RequestData::Hdel(Hdel {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    pub fn new_hexist(table: &str, key: &str) -> Self {
        Self {
            request_data: Some(RequestData::Hexist(Hexist {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
}
