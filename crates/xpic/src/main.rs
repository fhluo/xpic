use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, io};
use url::Url;
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

impl CLI {
    async fn run(self) {
        match self {
            CLI::List { number } => Self::list(number).await,
            CLI::Save { dir } => Self::save(dir).await,
        }
    }

    async fn list(number: Option<usize>) {
        match list_images(&Query::new().number(number.unwrap_or(8))).await {
            Ok(images) => {
                for image in images {
                    println!("{}: {}", image.title, image.url);
                }
            }
            Err(err) => eprintln!("failed to get Bing wallpapers: {err}"),
        }
    }

    async fn save(dir: impl AsRef<Path>) {
        if let Err(err) = copy_images_to(&dir).await {
            eprintln!(
                "failed to copy Bing wallpapers to {}:{}",
                dir.as_ref().display(),
                err
            );
        }
    }
}

#[tokio::main]
async fn main() {
    CLI::parse().run().await;
}

/// Downloads file from url to dst.
pub async fn download_file(url: &Url, dst: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    if dst.as_ref().exists() {
        return Ok(());
    }

    let resp = reqwest::get(url.as_ref()).await?;

    if !resp.status().is_success() {
        return Err(format!("failed to download file from {url}").into());
    }

    let mut file = File::create(dst)?;
    let content = resp.bytes().await?;
    io::copy(&mut content.as_ref(), &mut file)?;

    Ok(())
}

/// Copies images to a specified directory.
pub async fn copy_images_to(dst: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let dst = dst.as_ref();

    fs::create_dir_all(dst)
        .map_err(|err| format!("failed to create {}: {}", dst.display(), err))?;

    let tasks = list_images(&Query::new())
        .await?
        .into_iter()
        .filter_map(|image| {
            let dst = dst.join(image.id);
            if dst.exists() {
                return None;
            }

            Some(tokio::spawn(async move {
                if let Err(e) = download_file(&image.url, dst).await {
                    eprintln!("failed to download {}: {}", image.url, e);
                }
            }))
        });

    futures::future::join_all(tasks).await;
    Ok(())
}
