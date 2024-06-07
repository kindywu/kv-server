use redb::{Database, Error, TableDefinition};

const TABLE: TableDefinition<&str, u64> = TableDefinition::new("my_data");

#[cfg(not(target_os = "wasi"))]
fn main() -> Result<(), Error> {
    let file = tempfile::NamedTempFile::new().unwrap();
    let db = Database::create(file.path())?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TABLE)?;
        table.insert("my_key", &123)?;
    }
    write_txn.commit()?;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TABLE)?;
    assert_eq!(table.get("my_key")?.unwrap().value(), 123);

    Ok(())
}
