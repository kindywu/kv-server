use crate::pb::{CommandResponse, Hget};

use super::{CommandExecutor, KvError};

impl CommandExecutor for Hget {
    fn execute(self, store: &impl super::Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound {
                table: self.table,
                key: self.key,
            }
            .into(),
            Err(e) => e.into(),
        }
    }
}
