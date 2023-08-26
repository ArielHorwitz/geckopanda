use crate::{Backend, ObjectMetadata};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use google_drive3::{hyper, hyper_rustls, oauth2, DriveHub};
use google_drive3::api::{File, Scope};
use mime::Mime;
use oauth2::InstalledFlowAuthenticator as Authenticator;
use oauth2::InstalledFlowReturnMethod as ReturnMethod;
use std::fs;
use std::path::Path;
use std::io::Write;
use tokio::runtime::Runtime;
use json::{self, JsonValue};

type GoogleDriveHub = DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

async fn get_hub(
    client_secret: &Path,
    token_cache: &Path,
) -> Result<GoogleDriveHub> {
    let secret = get_secret(client_secret)?;
    let auth = Authenticator::builder(secret, ReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(token_cache)
        .build()
        .await?;
    auth.token(&["https://www.googleapis.com/auth/drive.file"]).await?;
    let client = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    );
    let hub = DriveHub::new(client, auth);
    // make any request to prompt oauth process
    hub.files().list().doit().await?;
    Ok(hub)
}

fn get_secret(client_secret: &Path) -> Result<oauth2::ApplicationSecret> {
    fn get_json_string(data: &json::object::Object, key: &str) -> Result<String> {
        match data.get(key).ok_or(anyhow!("missing {key}"))? {
            JsonValue::String(s) => Ok(s.to_owned()),
            JsonValue::Short(s) => Ok(s.as_str().to_owned()),
            o => Err(anyhow!("expected a string for {key}, got {o:?}")),
        }
    }

    fn get_json_array_strings(data: &json::object::Object, key: &str) -> Result<Vec<String>> {
        match data.get(key).ok_or(anyhow!("missing {key}"))? {
            JsonValue::Array(arr) => {
                let mut arr_str = Vec::new();
                for jv in arr {
                    arr_str.push(match jv {
                        JsonValue::String(s) => s.to_owned(),
                        JsonValue::Short(s) => s.as_str().to_owned(),
                        o => return Err(anyhow!("found non-string in array: {o:?}")),
                    });
                }
                Ok(arr_str)
            }
            o => Err(anyhow!("expected an array for {key}, got {o:?}")),
        }
    }

    let json_contents = fs::read_to_string(client_secret)?;
    let secret_data = match json::parse(&json_contents)? {
        JsonValue::Object(d) => Ok(d),
        _ => Err(anyhow!("missing data in client_secret json")),
    }?;
    let secret_data = match secret_data.get("installed") {
        Some(JsonValue::Object(d)) => Ok(d),
        _ => Err(anyhow!("missing data in client_secret json")),
    }?;
    let app_secret = oauth2::ApplicationSecret {
        client_id: get_json_string(secret_data, "client_id")?,
        client_secret: get_json_string(secret_data, "client_secret")?,
        token_uri: get_json_string(secret_data, "token_uri")?,
        auth_uri: get_json_string(secret_data, "auth_uri")?,
        redirect_uris: get_json_array_strings(secret_data, "redirect_uris")?,
        project_id: Some(get_json_string(secret_data, "project_id")?),
        client_email: None,
        auth_provider_x509_cert_url: Some(get_json_string(secret_data, "auth_provider_x509_cert_url")?),
        client_x509_cert_url: None,
    };
    Ok(app_secret)
}

#[derive(Clone)]
pub struct Storage {
    hub: GoogleDriveHub,
}

impl Storage {
    pub fn new(
    client_secret: &str,
    token_cache: &str,
) -> Result<Self> {
        let hub = get_hub(&Path::new(client_secret), &Path::new(token_cache));
        let hub = Runtime::new()?.block_on(hub)?;
        Ok(Self { hub })
    }
}

const FIELDS: &str = "files(id,name,modifiedTime,size,trashed,explicitlyTrashed)";

#[async_trait]
impl Backend for Storage {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let (_response, filelist) = self.hub
            .files()
            .list()
            .page_size(10)
            .param("fields", FIELDS)
            .doit()
            .await?;
        Ok(filelist
            .files
            .ok_or(anyhow!("missing filelist.files"))?
            .iter()
            .filter_map(|file| {
                if file.trashed? | file.explicitly_trashed? {
                    return None
                };
                Some(ObjectMetadata::new(
                    &file.id.clone()?,
                    &file.name.clone()?,
                    &file.modified_time?.to_string(),
                    file.size? as u64,
                ))
            })
            .collect())
    }

    async fn create(&self, file_name: &str) -> Result<String> {
        let mime_type = "application/octet-stream".parse::<Mime>()?;
        let mut file_metadata = File::default();
        file_metadata.name = Some(file_name.to_owned());
        let (_response, file) = self.hub
            .files()
            .create(file_metadata)
            .upload(tempfile::tempfile()?, mime_type)
            .await?;
        file.id.ok_or(anyhow!("missing file id"))
    }

    async fn get(&self, file_id: &str) -> Result<Vec<u8>> {
        let (response, _file) = self.hub
            .files()
            .get(file_id)
            .param("alt", "media")
            .add_scope(Scope::Full)
            .doit()
            .await?;
        let body = response.into_body();
        let bytes = hyper::body::to_bytes(body).await?;
        Ok(bytes.to_vec())
    }

    async fn update(&self, file_id: &str, data: &[u8]) -> Result<()> {
        let mut file = tempfile::tempfile()?;
        file.write(data)?;
        let mime_type = "application/octet-stream".parse::<Mime>()?;
        self.hub
            .files()
            .update(File::default(), file_id)
            .add_scope(Scope::Full)
            .upload(file, mime_type)
            .await?;
        Ok(())

    }
    async fn delete(&self, file_id: &str) -> Result<()> {
        self.hub
            .files()
            .delete(file_id)
            .add_scope(Scope::Full)
            .doit()
            .await?;
        Ok(())
    }
}

