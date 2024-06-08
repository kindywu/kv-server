use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use redb::{Database, TableDefinition, TypeName};
use std::path::Path;

use crate::{error::KvError, Kvpair, Storage, Value};

#[derive(Debug)]
pub struct RedbStorage {
    db: Database,
}

impl RedbStorage {
    pub fn try_new(file: &Path) -> Result<Self> {
        let db = Database::create(file)?;
        Ok(Self { db })
    }
}

#[allow(unused)]
impl Storage for RedbStorage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table: TableDefinition<&str, Value> = TableDefinition::new(table);

        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(table)?;
        Ok(table.get(key)?.map(|v| v.value()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table: TableDefinition<&str, Value> = TableDefinition::new(table);
        let key: &str = &key;

        let mut result = None;
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            result = table.insert(key, value)?.map(|v| v.value());
        }
        write_txn.commit()?;
        Ok(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table: TableDefinition<&str, Value> = TableDefinition::new(table);

        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(table)?;
        Ok(table.get(key)?.is_some())
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table: TableDefinition<&str, Value> = TableDefinition::new(table);

        let mut result = None;
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            result = table.remove(key)?.map(|v| v.value());
        }
        write_txn.commit()?;
        Ok(result)
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}

impl redb::Value for Value {
    type SelfType<'a> = Value;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let buf = Bytes::copy_from_slice(data);
        Value::decode(buf).expect("decode")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Vec<u8>
    where
        Self: 'a,
        Self: 'b,
    {
        value.to_bytes().expect("encode").to_vec()
    }

    fn type_name() -> TypeName {
        TypeName::new("abi::Value")
    }
}

#[cfg(test)]
mod tests {
    use tracing::debug;

    use super::*;

    #[test]
    fn get_or_create_table_should_work() -> Result<()> {
        let file = tempfile::NamedTempFile::new().unwrap();
        debug!("{file:?}");
        let storage = RedbStorage::try_new(file.path())?;
        storage.set("table", "key".to_string(), "value".into())?;
        let value = storage.get("table", "key")?;
        assert!(value.is_some());
        assert_eq!(value.unwrap(), "value".into());
        Ok(())
    }
}
