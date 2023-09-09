use crate::{ObjectMetadata, Storage};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use google_drive3::api::{File, Scope};
use google_drive3::{hyper, hyper_rustls, oauth2, DriveHub};
use mime::Mime;
use oauth2::authenticator_delegate::InstalledFlowDelegate;
use oauth2::parse_application_secret;
use oauth2::InstalledFlowAuthenticator as Authenticator;
use oauth2::InstalledFlowReturnMethod as ReturnMethod;
use std::future::Future;
use std::io::Write;
use std::path::Path;
use std::pin::Pin;
use tokio::runtime::Runtime;

/// ## Backend Setup
/// Create your Google Cloud `oauth2` [client secret](
/// https://console.cloud.google.com/apis/credentials) and download the client
/// secret file.
///
/// We can pass sensitive data via environment variables, or use the `inlcude_str!`
/// macro so that sensitive data is baked into the binary when built instead of
/// being distributed in a separate file.
///
/// ## Example
/// ```rust
/// use geckopanda::prelude::*;
/// let client_secret = include_str!("../../client_secret.json");
/// let token_cache = "../../token_cache.json";
/// let storage = GoogleDriveStorage::new(client_secret, token_cache).unwrap();
/// ```
#[derive(Clone)]
pub struct GoogleDriveStorage {
    hub: GoogleDriveHub,
}

impl GoogleDriveStorage {
    pub fn new(client_secret: &str, token_cache: &str) -> Result<Self> {
        let get_hub_coro = get_hub(client_secret, token_cache);
        let hub = Runtime::new()?.block_on(get_hub_coro)?;
        Ok(Self { hub })
    }
}

#[async_trait]
impl Storage for GoogleDriveStorage {
    async fn list(&self) -> Result<Vec<ObjectMetadata>> {
        let (_response, filelist) = self
            .hub
            .files()
            .list()
            .page_size(10)
            .param(
                "fields",
                "files(id,name,modifiedTime,size,trashed,explicitlyTrashed)",
            )
            .add_scope(Scope::File)
            .doit()
            .await?;
        Ok(filelist
            .files
            .ok_or(anyhow!("missing filelist.files"))?
            .iter()
            .filter_map(|file| {
                if file.trashed? | file.explicitly_trashed? {
                    return None;
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
        let (_response, file) = self
            .hub
            .files()
            .create(file_metadata)
            .add_scope(Scope::File)
            .upload(tempfile::tempfile()?, mime_type)
            .await?;
        file.id.ok_or(anyhow!("missing file id"))
    }

    async fn get(&self, file_id: &str) -> Result<Vec<u8>> {
        let (response, _file) = self
            .hub
            .files()
            .get(file_id)
            .param("alt", "media")
            .add_scope(Scope::File)
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
            .add_scope(Scope::File)
            .upload(file, mime_type)
            .await?;
        Ok(())
    }

    async fn delete(&self, file_id: &str) -> Result<()> {
        self.hub
            .files()
            .delete(file_id)
            .add_scope(Scope::File)
            .doit()
            .await?;
        Ok(())
    }
}

type GoogleDriveHub = DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

async fn get_hub(client_secret: &str, token_cache: &str) -> Result<GoogleDriveHub> {
    let secret = parse_application_secret(client_secret)?;
    let auth = Authenticator::builder(secret, ReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(Path::new(token_cache))
        .flow_delegate(Box::new(BrowserDelegate))
        .build()
        .await?;
    let scopes = &["https://www.googleapis.com/auth/drive.file"];
    auth.token(scopes).await?;
    let client = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    );
    let hub = DriveHub::new(client, auth);
    Ok(hub)
}

#[derive(Copy, Clone)]
struct BrowserDelegate;

impl InstalledFlowDelegate for BrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(browser_delegate(url, need_code))
    }
}

async fn browser_delegate(url: &str, need_code: bool) -> Result<String, String> {
    if need_code {
        unimplemented!("manual oauth2 user input not supported");
    }
    webbrowser::open(url).map_err(|e| format!("{e:?}"))?;
    Ok(String::new())
}
