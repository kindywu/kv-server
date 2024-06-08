use crate::{
    error::KvError,
    pb::{CommandResponse, Hget},
};

use super::CommandExecutor;

impl CommandExecutor for Hget {
    fn execute(self, store: &dyn super::Storage) -> CommandResponse {
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
