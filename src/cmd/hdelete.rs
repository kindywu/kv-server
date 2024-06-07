use crate::pb::Hdel;

use super::CommandExecutor;

impl CommandExecutor for Hdel {
    fn execute(self, _store: &impl super::Storage) -> crate::pb::CommandResponse {
        todo!()
    }
}
