use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use kv_server::{RedbStorage, Storage};

fn redb_storage_test(storage: &impl Storage) -> Result<()> {
    storage.set("table", "key".to_string(), "value".into())?;
    let value = storage.get("table", "key")?;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value".into());
    Ok(())
}

fn redb_storage_benchmark(c: &mut Criterion) {
    let file = tempfile::NamedTempFile::new().unwrap();
    let storage = RedbStorage::try_new(file.path()).unwrap();

    c.bench_function("redb storage", |b| b.iter(|| redb_storage_test(&storage)));
}

criterion_group!(benches, redb_storage_benchmark);
criterion_main!(benches);
