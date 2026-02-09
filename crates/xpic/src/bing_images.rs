use crate::bing::{list_images, Query};
use crate::image::Image;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::{fs, io};
use url::Url;

pub async fn get_images() -> Result<Vec<Url>, Box<dyn Error>> {
    Ok(list_images(&Query::new())
        .await?
        .into_iter()
        .filter_map(|info| Image::try_from(info).ok())
        .map(|image| image.url)
        .collect::<Vec<_>>())
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
            let image = Image::try_from(image).ok()?;
            let dst = dst.join(image.id()?);
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
