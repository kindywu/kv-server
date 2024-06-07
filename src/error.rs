use http::StatusCode;
use thiserror::Error;

use crate::CommandResponse;

#[derive(Error, Debug)]
pub enum KvError {
    #[error("table {table} key {key} not found")]
    NotFound { table: String, key: String },

    #[error("invalid command: {0}")]
    InvalidCommand(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<KvError> for CommandResponse {
    fn from(error: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: error.to_string(),
            ..Default::default()
        };
        match error {
            KvError::NotFound { table: _, key: _ } => {
                result.status = StatusCode::NOT_FOUND.as_u16() as _
            }
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => (),
        };
        result
    }
}
