use crate::CLI::{Download, Export, List};
use anyhow::anyhow;
use clap::Parser;
use futures::StreamExt;
use reqwest::IntoUrl;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xpic::bing::Market;
use xpic::{list_images, Image, Query};

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
    /// Download recent wallpapers to a directory
    Download {
        /// The directory where wallpapers are saved
        dir: PathBuf,
    },
    /// Export wallpaper metadata to JSON files
    Export {
        /// The directory where metadata is saved
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
        Download { dir } => {
            download_wallpapers(&dir)
                .await
                .map_err(|err| anyhow!("failed to save wallpapers: {err}"))?;
        }
        Export { dir } => {
            export_metadata(&dir).await?;
        }
    }

    Ok(())
}

pub async fn download_file(url: impl IntoUrl, path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
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

pub async fn download_wallpapers(dir: impl AsRef<Path>) -> Result<(), anyhow::Error> {
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
                let result = download_file(image.url.clone(), path)
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

pub async fn update_metadata_file(
    path: impl AsRef<Path>,
    mut images: Vec<Image>,
) -> Result<(), anyhow::Error> {
    let path = path.as_ref();

    if path.exists() {
        let file = tokio::fs::File::open(path).await?;
        let mut reader = tokio::io::BufReader::new(file);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;

        images.extend(serde_json::from_slice::<Vec<Image>>(&buffer)?);
    }

    images.sort_by(|a, b| a.hash.cmp(&b.hash));
    images.dedup_by(|a, b| a.hash == b.hash);

    images.sort_by(|a, b| b.start_date.cmp(&a.start_date));

    tokio::fs::write(path, serde_json::to_vec_pretty(&images)?).await?;

    Ok(())
}

pub async fn export_metadata(dir: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    let dir = dir.as_ref();

    tokio::fs::create_dir_all(dir).await?;

    // ROW: Rest of the World
    let mut market_images: HashMap<String, Vec<Image>> = HashMap::new();

    for market in Market::iter() {
        let images = list_images(&Query::new().market(market)).await?;

        for image in images {
            if let Some(id) = image.id_parsed.as_ref() {
                market_images
                    .entry(id.market.clone())
                    .or_default()
                    .push(image);
            }
        }
    }

    for (market, images) in market_images {
        let mut path = dir.join(market);
        path.set_extension("json");

        update_metadata_file(path, images).await?;
    }

    Ok(())
}
