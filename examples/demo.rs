use std::sync::Arc;
use std::thread;

trait Storage: Send + Sync {
    fn store(&self);
}

struct MemoryStorage;

impl Storage for MemoryStorage {
    fn store(&self) {
        // 实现细节
    }
}

fn main() {
    let storage = MemoryStorage;
    let storage: Arc<dyn Storage> = Arc::new(storage);

    let handle = thread::spawn(move || {
        storage.store();
    });

    handle.join().unwrap();
}
