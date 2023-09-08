use anyhow::Result;
use geckopanda::prelude::*;

fn main() -> Result<()> {
    let storage = LocalDiskStorage::new("../storagecache")?;
    println!("created local disk storage");

    let file_id = storage.create_sync("example.file")?;
    println!("created file id {file_id}");

    let data = "example file content";
    storage.update_sync(&file_id, data.as_bytes())?;
    println!("uploaded data: {data:?}");

    let drive_data = String::from_utf8(storage.get_sync(&file_id)?)?;
    assert_eq!(data, &drive_data);
    println!("downloaded data: {drive_data:?}");

    storage.delete_sync(&file_id)?;
    println!("deleted file id {file_id}");
    Ok(())
}
