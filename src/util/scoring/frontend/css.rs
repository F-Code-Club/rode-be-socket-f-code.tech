use anyhow::{bail, Context};
use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams,
};
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{handler::viewport::Viewport, Browser, BrowserConfig};
use futures::StreamExt;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageFormat};
use pixelmatch::pixelmatch;

use std::fs::metadata;
use std::io::Cursor;

use crate::config;
use crate::database::model::Template;
use crate::util::drive::HubDrive;
use crate::util::scoring::{ExecutionResult, ResultKind};

async fn render_image(code: &str, width: u32, height: u32) -> anyhow::Result<Vec<u8>> {
    let viewport = Viewport {
        width,
        height,
        ..Default::default()
    };
    let config = match BrowserConfig::builder()
        .viewport(viewport)
        .chrome_executable(
            config::CHROME_PATH
                .get()
                .context("Failed to get chrome path")?,
        )
        .no_sandbox()
        .build()
    {
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

#[tracing::instrument(err)]
pub async fn render_diff_image(
    question_image_buffer: &[u8],
    html: String,
) -> anyhow::Result<(f32, Vec<u8>)> {
    let question_image =
        image::load_from_memory_with_format(question_image_buffer, ImageFormat::Png)?;
    let (width, height) = question_image.dimensions();
    let answer_image_buffer = render_image(&html, width, height).await?;

    let mut diff_image_buffer = Vec::with_capacity((height * width) as usize);

    match pixelmatch(
        question_image_buffer,
        answer_image_buffer.as_slice(),
        Some(&mut diff_image_buffer),
        Some(width),
        Some(height),
        Some(pixelmatch::Options {
            threshold: 0.1,
            ..Default::default()
        }),
    ) {
        Err(error) => {
            bail!(format!("{}", error))
        }
        Ok(diff) => {
            let match_percent = (1. - (diff as f32) / ((width * height) as f32)) * 100.;
            Ok((match_percent, diff_image_buffer))
        }
    }
}

#[tracing::instrument(err)]
pub async fn execute(code: &str, template: Template) -> anyhow::Result<ExecutionResult> {
    let mut template_buffer = template.download().await?;

    let (percent, _) = render_diff_image(&template_buffer, code.to_owned()).await?;

    Ok(ExecutionResult {
        score: percent as u32,
        run_time: 0,
        details: None,
        compilation_error: None,
        kind: ResultKind::Executed,
    })
}
