use anyhow::Ok;
use google_drive3::hyper::client::HttpConnector;
use google_drive3::hyper_rustls::HttpsConnector;
use google_drive3::oauth2::ApplicationSecret;
use google_drive3::{DriveHub, oauth2, hyper, hyper_rustls};

use google_drive3::oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::future::Future;
use std::pin::Pin;
use hyper::body::to_bytes;

use crate::config ;

// Extract {file-id} after /d/ in "https://drive.google.com/file/d/{file-id}/" to download the specific file.
fn extract_file_id(url: &str) -> Option<&str> {
    let parts: Vec<&str> = url.split('/').collect();
    if let Some(pos) = parts.iter().position(|&s| s == "d") {
        parts.get(pos + 1).copied()
    } else {
        None
    }
}
pub struct HubDrive {
    instance: DriveHub<HttpsConnector<HttpConnector>>,
}
impl HubDrive{
    pub async fn new() -> anyhow::Result<Self>{
        // Put your client secret in the working directory!
        let secret = ApplicationSecret {
            client_id: config::GOOGLE_CLIENT_ID.to_string(),
            client_secret: config::GOOGLE_CLIENT_SECRET.to_string(),
            redirect_uris: vec![config::GOOGLE_REDIRECT_URL.to_string()],
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_owned()),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            ..Default::default()
        };
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPPortRedirect(*config::SERVER_PORT),
        )
        .build()
        .await?;
    
        let instance = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );
        Ok(Self { instance })
    }

    pub async fn download_file_by_id(&self, url_path: &str, local_path: &str) -> anyhow::Result<()>{
        let file_id = extract_file_id(url_path).unwrap();
        let (resposne, _) = self.instance
        .files()
        .get(file_id)
        .param("alt","media")
        .add_scope(google_drive3::api::Scope::Full)
        .doit().await?;

        let mut file = File::create(local_path).await?;
        // Turn response body to byte and then write to file
        file.write_all(&to_bytes(resposne.into_body()).await?).await?;
        Ok(())
    }
}
