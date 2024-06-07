use std::ops::Deref;

use crate::{Storage, Value};
use dashmap::DashMap;

pub struct MemoryStorage {
    inner_storage: DashMap<String, DashMap<String, Value>>,
}

impl Deref for MemoryStorage {
    type Target = DashMap<String, DashMap<String, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.inner_storage
    }
}

impl MemoryStorage {
    pub fn new(inner_storage: DashMap<String, DashMap<String, Value>>) -> Self {
        Self { inner_storage }
    }

    fn get_table(
        &self,
        table: &str,
    ) -> dashmap::mapref::one::RefMut<String, DashMap<String, Value>, std::hash::RandomState> {
        let table = self.entry(table.into()).or_default();
        table
    }
}

#[allow(unused)]
impl Storage for MemoryStorage {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::error::KvError> {
        let table = self.get_table(table);
        Ok(table.get(key).map(|v| v.clone()))
    }

    fn set(
        &self,
        table: &str,
        key: String,
        value: crate::Value,
    ) -> Result<Option<crate::Value>, crate::error::KvError> {
        let table = self.get_table(table);
        Ok(table.insert(key, value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::error::KvError> {
        let table = self.get_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::error::KvError> {
        let table = self.get_table(table);
        Ok(table.remove(key).map(|v| v.1.clone()))
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::error::KvError> {
        todo!()
    }
}
