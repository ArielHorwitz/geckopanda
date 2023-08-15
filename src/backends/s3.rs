use crate::{Backend, ObjectMetadata};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use s3::creds::Credentials;
use s3::request::request_trait::ResponseData;
use s3::{Bucket, Region};

#[derive(Clone, Debug)]
pub struct Storage {
    bucket: Bucket,
}

impl Storage {
    pub fn new(
        bucket_name: &str,
        region: &str,
        secret_key: &str,
        access_key: &str,
    ) -> Result<Self> {
        let region = region.parse()?;
        let creds = Credentials::new(Some(secret_key), Some(access_key), None, None, None)?;
        let bucket = Bucket::new(bucket_name, region, creds)?;
        Ok(Self { bucket })
    }

    // Provided as a workaround for rust-s3 missing region
    pub fn new_israel(bucket_name: &str, secret_key: &str, access_key: &str) -> Result<Self> {
        let region = Region::Custom {
            region: "il-central-1".to_owned(),
            endpoint: "s3.il-central-1.amazonaws.com".to_owned(),
        };
        let creds = Credentials::new(Some(secret_key), Some(access_key), None, None, None)?;
        let bucket = Bucket::new(bucket_name, region, creds)?;
        Ok(Self { bucket })
    }
}

#[async_trait]
impl Backend for Storage {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let mut listing = self.bucket.list("".to_owned(), None).await?;
        let contents = listing.pop().expect("expected a single result").contents;
        let objects = contents
            .iter()
            .map(|o| ObjectMetadata {
                name: o.key.to_owned(),
                last_modified: o.last_modified.to_owned(),
                size: o.size.to_owned(),
            })
            .collect();
        Ok(objects)
    }

    async fn get(&self, file: &str) -> Result<Vec<u8>> {
        let object = self.bucket.get_object(file).await?;
        check_status(&object)?;
        Ok(object.to_vec())
    }

    async fn put(&self, file: &str, data: &[u8]) -> Result<()> {
        let response = self.bucket.put_object(file, data).await?;
        check_status(&response)?;
        Ok(())
    }

    async fn delete(&self, file: &str) -> Result<()> {
        let response = self.bucket.delete_object(file).await?;
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
