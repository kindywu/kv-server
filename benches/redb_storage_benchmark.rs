use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use kv_server::{MemoryStorage, Storage};

fn memory_storage_test(storage: &impl Storage) -> Result<()> {
    storage.set("table", "key".to_string(), "value".into())?;
    let value = storage.get("table", "key")?;
    assert!(value.is_some());
    assert_eq!(value.unwrap(), "value".into());
    Ok(())
}

fn memory_storage_benchmark(c: &mut Criterion) {
    let storage = MemoryStorage::new();

    c.bench_function("memory storage", |b| {
        b.iter(|| memory_storage_test(&storage))
    });
}

criterion_group!(benches, memory_storage_benchmark);
criterion_main!(benches);
