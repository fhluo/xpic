use crate::CLI::{List, Save};
use anyhow::anyhow;
use clap::Parser;
use futures::StreamExt;
use reqwest::IntoUrl;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use xpic::{list_images, Query};

/// Bing wallpapers
#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
enum CLI {
    /// List Bing wallpapers
    List {
        /// The number of wallpapers to list
        #[arg(short)]
        number: Option<usize>,
    },
    /// Save wallpapers to a directory
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = CLI::parse();

    match cli {
        List { number } => {
            let images = list_images(&Query::new().number(number.unwrap_or(8)))
                .await
                .map_err(|err| anyhow!("failed to list wallpapers: {err}"))?;

            for image in images {
                println!("{}: {}", image.title, image.url);
            }
        }
        Save { dir } => {
            save_wallpapers(&dir)
                .await
                .map_err(|err| anyhow!("failed to save wallpapers: {err}"))?;
        }
    }

    Ok(())
}

pub async fn download(url: impl IntoUrl, path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    if path.as_ref().exists() {
        return Ok(());
    }

    let resp = reqwest::get(url).await?.error_for_status()?;

    let mut file = tokio::fs::File::create(path).await?;
    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;
    }

    Ok(())
}

pub async fn save_wallpapers(dir: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    let dir = dir.as_ref();

    tokio::fs::create_dir_all(dir).await?;

    let tasks = list_images(&Query::new())
        .await?
        .into_iter()
        .filter_map(|image| {
            let path = dir.join(image.id);
            if path.exists() {
                return None;
            }

            Some(tokio::spawn(async move {
                let result = download(image.url.clone(), path)
                    .await
                    .map_err(|err| anyhow!("download failed: {err}"));

                if let Err(err) = result {
                    eprintln!("{err}");
                }
            }))
        });

    futures::future::join_all(tasks).await;
    Ok(())
}
