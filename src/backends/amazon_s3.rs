use crate::{ObjectMetadata, Storage};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use s3::creds::Credentials;
use s3::request::request_trait::ResponseData;
use s3::{Bucket, Region};
use serde::Deserialize;
use toml;

/// ## Backend Setup
/// Create a new user in your [AWS IAM console](https://console.aws.amazon.com/iam)
/// and add an inline permission policy (see the `s3permissions-template.json`
/// file). Then we create an access key and fill in the details of the
/// `s3config-template.toml` file.
///
/// We can pass sensitive data via environment variables, or use the `inlcude_str!`
/// macro so that sensitive data is baked into the binary when built instead of
/// being distributed in a separate file.
///
/// ## Example
/// ```rust
/// use geckopanda::prelude::*;
/// let config = include_str!("../../s3config.toml");
/// let storage = AmazonS3Storage::new(config).unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct AmazonS3Storage {
    bucket: Bucket,
}

#[derive(Deserialize)]
struct AmazonS3Config {
    bucket_name: String,
    region: String,
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
}

impl AmazonS3Storage {
    pub fn new(config_data: &str) -> Result<Self> {
        let config: AmazonS3Config = toml::from_str(config_data)?;
        let region = if config.endpoint.is_empty() {
            config.region.parse()?
        } else {
            Region::Custom {
                region: config.region,
                endpoint: config.endpoint,
            }
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
impl Storage for AmazonS3Storage {
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
