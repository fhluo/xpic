use gpui::{AssetSource, ImageSource, Resource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
#[include = "app-icon.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

pub enum Icon {
    AppIcon,
}

impl Icon {
    fn path(&self) -> &'static str {
        match self {
            Self::AppIcon => "app-icon.svg",
        }
    }
}

impl From<Icon> for ImageSource {
    fn from(icon: Icon) -> Self {
        ImageSource::Resource(Resource::Embedded(icon.path().into()))
    }
}
