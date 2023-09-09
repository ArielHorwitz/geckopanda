use anyhow::Result;
use geckopanda::prelude::*;

fn main() -> Result<()> {
    let client_secret = include_str!("../client_secret.json");
    let storage = GoogleDriveStorage::new(client_secret, "../token_cache.json")?;
    println!("created google drive storage");

    let file_id = storage.create_blocking("example.file")?;
    println!("created file id {file_id}");

    let data = "example file content";
    storage.update_blocking(&file_id, data.as_bytes())?;
    println!("uploaded data: {data:?}");

    let drive_data = String::from_utf8(storage.get_blocking(&file_id)?)?;
    assert_eq!(data, &drive_data);
    println!("downloaded data: {drive_data:?}");

    storage.delete_blocking(&file_id)?;
    println!("deleted file id {file_id}");
    Ok(())
}
