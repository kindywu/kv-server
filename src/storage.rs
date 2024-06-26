mod memory;
mod redb;

pub use memory::*;
pub use redb::*;

use crate::{error::KvError, Kvpair, Value};
use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Storage: Send + Sync {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从 HashTable 中删除一个 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[enum_dispatch(Storage)]
pub enum StorageState {
    MemoryStorage(MemoryStorage),
    RedbStorage(RedbStorage),
}
