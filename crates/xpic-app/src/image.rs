use crate::config::Config;
use crate::RUNTIME;
use gpui::{
    img, App, Asset, ImageCacheError, ImageSource, Img, IntoElement, RenderImage, SharedString,
};
use image::RgbaImage;
use photon_rs::colour_spaces::lighten_hsl;
use photon_rs::PhotonImage;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use xpic::bing::{ThumbnailParams, ThumbnailQuery, UrlBuilder};

#[derive(Debug, Clone)]
pub struct Image {
    url: UrlBuilder,
    lighten_level: Option<f32>,
}

impl Image {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            url: UrlBuilder::new(id),
            lighten_level: None,
        }
    }

    pub fn lighten_level(mut self, level: f32) -> Self {
        self.lighten_level = Some(level);

        self
    }

    fn rgba_to_bgra(mut img: RgbaImage) -> RgbaImage {
        for pixel in img.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }

        img
    }

    fn lighten(img: RgbaImage, level: f32) -> RgbaImage {
        let (w, h) = img.dimensions();

        let mut photon_img = PhotonImage::new(img.into_raw(), w, h);
        lighten_hsl(&mut photon_img, level);

        RgbaImage::from_raw(w, h, photon_img.get_raw_pixels()).unwrap()
    }

    pub fn decode(
        bytes: impl AsRef<[u8]>,
        lighten_level: Option<f32>,
    ) -> Result<Arc<RenderImage>, ImageCacheError> {
        let mut img = image::load_from_memory(bytes.as_ref())
            .map_err(|err| ImageCacheError::Other(Arc::new(err.into())))?
            .into_rgba8();

        if let Some(lighten_level) = lighten_level {
            img = Self::lighten(img, lighten_level)
        }

        Ok(Arc::new(RenderImage::new([image::Frame::new(
            Self::rgba_to_bgra(img),
        )])))
    }
}

impl ThumbnailParams for Image {
    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        self.url.query_mut()
    }
}

#[derive(Debug, Clone)]
pub struct ImageAssetSource {
    url: SharedString,
    lighten_level: Option<f32>,
}

impl Hash for ImageAssetSource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.lighten_level.map(f32::to_bits).hash(state);
    }
}

impl IntoElement for Image {
    type Element = Img;

    fn into_element(self) -> Self::Element {
        let source = ImageAssetSource {
            url: self.url.build().unwrap().into(),
            lighten_level: self.lighten_level,
        };

        img(ImageSource::Custom(Arc::new(move |window, cx| {
            window.use_asset::<Image>(&source, cx)
        })))
    }
}

impl Asset for Image {
    type Source = ImageAssetSource;
    type Output = Result<Arc<RenderImage>, ImageCacheError>;

    fn load(
        ImageAssetSource { url, lighten_level }: Self::Source,
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

            Image::decode(bytes, lighten_level)
        }
    }
}
