use anyhow::anyhow;
use git2::{ErrorCode, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, redirect::Policy};
use std::fs::remove_dir_all;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

pub async fn download_http(
    file_path: &str,
    app_name: &str,
    address: &str,
) -> Result<(), anyhow::Error> {
    let custom = Policy::custom(|attempt| {
        if attempt.previous().len() > 5 {
            attempt.error("too many redirects")
        } else if attempt.url().host_str() == Some("example.domain") {
            // prevent redirects to 'example.domain'
            attempt.stop()
        } else {
            attempt.follow()
        }
    });
    let client = reqwest::Client::builder().redirect(custom).build()?;

    let total_size = {
        let resp = client.head(address).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                address,
                resp.status(),
            ));
        }
    };

    let mut request = client.get(address);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "=> {app} {bar}",
                app = app_name,
                bar = "{wide_msg}[{bar:60.green/blue}] {percent:>3}% {total_bytes:>10}"
            ))
            .progress_chars("#>-"),
    );

    let file = Path::new(file_path);

    if file.exists() {
        let size = file.metadata()?.len().saturating_sub(1);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = request.send().await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .await?;

    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
        dest.flush().await?;
    }

    pb.finish();

    Ok(())
}

pub fn download_git(url: &str, clone_to: &str) {
    match Repository::clone(url, clone_to) {
        Ok(repo) => repo,
        Err(e) => match e.code() {
            ErrorCode::Exists => {
                remove_dir_all(clone_to).unwrap();
                Repository::clone(url, clone_to).unwrap()
            }
            _ => panic!("Failed to clone repository"),
        },
    };
}
