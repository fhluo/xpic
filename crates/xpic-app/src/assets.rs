use gpui::{AssetSource, ImageSource, Resource, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
#[include = "app-icon.png"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = Self::get(path) {
            Ok(Some(file.data))
        } else {
            gpui_component_assets::Assets.load(path)
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut paths = gpui_component_assets::Assets.list(path).unwrap_or_default();

        paths.extend(Self::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));

        Ok(paths)
    }
}

pub enum Icon {
    AppIcon,
}

impl Icon {
    fn path(&self) -> &'static str {
        match self {
            Self::AppIcon => "app-icon.png",
        }
    }
}

impl From<Icon> for ImageSource {
    fn from(icon: Icon) -> Self {
        ImageSource::Resource(Resource::Embedded(icon.path().into()))
    }
}

impl From<Icon> for SharedString {
    fn from(icon: Icon) -> Self {
        icon.path().into()
    }
}
