use crate::{Storage, ObjectMetadata};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use s3::creds::Credentials;
use s3::request::request_trait::ResponseData;
use s3::{Bucket, Region};
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Debug)]
pub struct Backend {
    bucket: Bucket,
}

#[derive(Deserialize)]
struct S3Config {
   bucket_name: String,
   region: String,
   endpoint: String,
   access_key_id: String,
   access_key_secret: String,
}

impl Backend {
    pub fn new(config_file: &str) -> Result<Self> {
        let config: S3Config = toml::from_str(&fs::read_to_string(config_file)?)?;
        let region = match config.endpoint.is_empty() {
            false => Region::Custom { region: config.region, endpoint: config.endpoint },
            true => config.region.parse()?,
        };
        let creds = Credentials::new(
            Some(&config.access_key_id),
            Some(&config.access_key_secret),
            None,
            None,
            None,
        )?;
        let bucket = Bucket::new(&config.bucket_name, region, creds)?;
        Ok(Self { bucket })
    }
}

#[async_trait]
impl Storage for Backend {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let mut listing = self.bucket.list("".to_owned(), None).await?;
        let contents = listing.pop().expect("expected a single result").contents;
        let objects = contents
            .iter()
            .map(|o| ObjectMetadata {
                id: o.key.to_owned(),
                name: o.key.to_owned(),
                last_modified: o.last_modified.to_owned(),
                size: o.size.to_owned(),
            })
            .collect();
        Ok(objects)
    }

    async fn create(&self, file_id: &str) -> Result<String> {
        self.update(file_id, "".as_bytes()).await?;
        Ok(file_id.to_owned())
    }

    async fn get(&self, file_id: &str) -> Result<Vec<u8>> {
        let object = self.bucket.get_object(file_id).await?;
        check_status(&object)?;
        Ok(object.to_vec())
    }

    async fn update(&self, file_id: &str, data: &[u8]) -> Result<()> {
        let response = self.bucket.put_object(file_id, data).await?;
        check_status(&response)?;
        Ok(())
    }

    async fn delete(&self, file_id: &str) -> Result<()> {
        let response = self.bucket.delete_object(file_id).await?;
        check_status(&response)?;
        Ok(())
    }
}

#[allow(dead_code)]
fn check_status(response: &ResponseData) -> Result<()> {
    match response.status_code() >= 300 {
        false => Ok(()),
        true => Err(anyhow!(response.status_code())),
    }
}
