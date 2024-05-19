use anyhow::Ok;
use google_drive3::hyper::client::HttpConnector;
use google_drive3::hyper_rustls::HttpsConnector;
use google_drive3::oauth2::ApplicationSecret;
use google_drive3::{DriveHub, oauth2, hyper, hyper_rustls};

use google_drive3::oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};
use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use hyper::body::to_bytes;

use crate::config ;

async fn browser_user_url(url: &str, need_code: bool) -> Result<String, String> {
    if webbrowser::open(url).is_ok() {
        println!("webbrowser was successfully opened.");
    }
    let def_delegate = DefaultInstalledFlowDelegate;
    def_delegate.present_user_url(url, need_code).await
}

#[derive(Copy, Clone)]
struct InstalledFlowBrowserDelegate;
impl InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    /// the actual presenting of URL and browser opening happens in the function defined above here
    /// we only pin it
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(browser_user_url(url, need_code))
    }
}

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
        .flow_delegate(Box::new(InstalledFlowBrowserDelegate))
        .build()
        .await
        .expect("InstalledFlowAuthenticator failed to build");
    
        let instance = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
        Ok(Self { instance })
    }

    pub async fn download_file_by_id(&self, url_path: &str, local_path: &str) -> anyhow::Result<()>{
        let file_id = extract_file_id(url_path).unwrap();
        let result = self.instance
        .files()
        .get(file_id)
        .param("alt","media")
        .add_scope(google_drive3::api::Scope::Full)
        .doit().await?;

        match result {
            (res,_) => {
                match to_bytes(res.into_body()).await? {
                    body_bytes => {
                        let mut file = File::create(local_path).expect("Failed to create file");
                        file.write_all(&body_bytes).expect("Failed to write to file");
                        Ok(())
                    }
                }
            }
        }
    }
}
