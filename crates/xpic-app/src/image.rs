use crate::config::Config;
use crate::RUNTIME;
use gpui::{
    img, prelude::*, App, Asset, ImageCacheError, ImageSource, Img, IntoElement, RenderImage,
    SharedString,
};
use image::ImageReader;
use std::io::Cursor;
use std::sync::Arc;
use xpic::bing::{ThumbnailParams, ThumbnailQuery, UrlBuilder};

pub struct Image {
    url: UrlBuilder,
}

impl Image {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            url: UrlBuilder::new(id),
        }
    }

    pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Arc<RenderImage>, ImageCacheError> {
        let mut data = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|err| ImageCacheError::Other(Arc::new(err.into())))?
            .decode()
            .map_err(|err| ImageCacheError::Other(Arc::new(err.into())))?
            .into_rgba8();

        for pixel in data.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        Ok(Arc::new(RenderImage::new([image::Frame::new(data)])))
    }
}

impl ThumbnailParams for Image {
    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        self.url.query_mut()
    }
}

impl IntoElement for Image {
    type Element = Img;

    fn into_element(self) -> Self::Element {
        let url = self.url.build().unwrap().into();

        img(ImageSource::Custom(Arc::new(move |window, cx| {
            window.use_asset::<Image>(&url, cx)
        })))
    }
}

impl Asset for Image {
    type Source = SharedString;
    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        url: Self::Source,
        cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let path = cx.global::<Config>().image_cache(&url);
        let handle = RUNTIME.handle().clone();

        async move {
            let bytes = handle
                .spawn(async move {
                    if path.exists() {
                        return tokio::fs::read(&path).await;
                    }

                    let data = reqwest::get(url.as_ref())
                        .await
                        .and_then(|r| r.error_for_status())
                        .map_err(|err| std::io::Error::other(err))?
                        .bytes()
                        .await
                        .map_err(|err| std::io::Error::other(err))?;

                    if let Some(dir) = path.parent() {
                        let _ = tokio::fs::create_dir_all(dir).await;
                    }
                    let _ = tokio::fs::write(&path, &data).await;

                    Ok(data.to_vec())
                })
                .await
                .map_err(|err| ImageCacheError::Other(Arc::new(err.into())))?
                .map_err(|err| ImageCacheError::Io(Arc::new(err)))?;

            Image::decode(bytes)
        }
    }
}
