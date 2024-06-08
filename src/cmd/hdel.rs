use crate::{error::KvError, Hdel};

use super::CommandExecutor;

impl CommandExecutor for Hdel {
    fn execute(self, store: &dyn super::Storage) -> crate::pb::CommandResponse {
        match store.del(&self.table, &self.key) {
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
