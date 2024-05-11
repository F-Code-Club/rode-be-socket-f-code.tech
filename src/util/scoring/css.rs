use anyhow::bail;
use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams,
};
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{handler::viewport::Viewport, Browser, BrowserConfig};
use futures::StreamExt;

use super::ExecutionResult;
use crate::database::model::Template;

#[allow(dead_code)]
async fn render_image(code: &str, width: u32, height: u32) -> anyhow::Result<Vec<u8>> {
    let viewport = Viewport {
        width,
        height,
        ..Default::default()
    };
    let config = match BrowserConfig::builder().viewport(viewport).build() {
        Ok(config) => config,
        Err(error) => {
            bail!(error)
        }
    };
    let (mut browser, mut handler) = Browser::launch(config).await?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser.new_page("about:blank").await?;
    page.set_content(code).await?;

    let image_buffer = page
        .screenshot(ScreenshotParams {
            cdp_params: CaptureScreenshotParams {
                format: Some(CaptureScreenshotFormat::Png),
                ..Default::default()
            },
            ..Default::default()
        })
        .await?;

    browser.close().await?;
    browser.wait().await?;
    handle.await?;

    Ok(image_buffer)
}

#[allow(unused_variables)]
pub async fn execute(code: &str, template: Template) -> anyhow::Result<ExecutionResult> {
    todo!()
}
