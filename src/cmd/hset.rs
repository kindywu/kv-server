use crate::{error::KvError, pb::Hset};

use super::CommandExecutor;

impl CommandExecutor for Hset {
    fn execute(self, store: &dyn super::Storage) -> crate::pb::CommandResponse {
        match self.pair {
            Some(pair) => match store.set(&self.table, pair.key, pair.value.unwrap_or_default()) {
                Ok(v) => v.into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into(),
        }
    }
}
