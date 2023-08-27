use anyhow::{anyhow, Result};
use geckopanda::{Storage, S3Storage};

fn main() -> Result<()> {
    /* We use the [include_str] macro to insert access key secrets directly into
    binary. Environment variables can also be used to insert secrets. */
    let config_data = include_str!("../s3config.toml");
    let storage = S3Storage::new(config_data)
        .map_err(|e| anyhow!("failed to load config: {e:?}"))?;
    let file_id = storage.create_sync("example.file")?;
    println!("created file id {file_id}");
    let data = "example file content";
    storage.update_sync(&file_id, data.as_bytes())?;
    println!("uploaded data: {data:?}");
    let drive_data = String::from_utf8(storage.get_sync(&file_id)?)?;
    println!("downloaded data: {drive_data:?}");
    assert_eq!(data, &drive_data);
    storage.delete_sync(&file_id)?;
    println!("deleted file id {file_id}");
    Ok(())
}

