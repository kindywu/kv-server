use crate::{Hexist, Value};

use super::CommandExecutor;

impl CommandExecutor for Hexist {
    fn execute(self, store: &impl super::Storage) -> crate::pb::CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => Value::from(v).into(),
            Err(e) => e.into(),
        }
    }
}
