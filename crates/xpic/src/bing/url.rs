use super::{CropMode, THUMBNAIL_URL};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct UrlBuilder {
    pub id: String,

    pub pid: Option<String>,

    #[serde(rename = "w")]
    pub width: Option<u32>,

    #[serde(rename = "h")]
    pub height: Option<u32>,

    #[serde(rename = "p")]
    pub padding: Option<u32>,

    #[serde(rename = "c")]
    pub crop: Option<CropMode>,
}

impl UrlBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            width: None,
            height: None,
            pid: None,
            padding: None,
            crop: None,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);

        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);

        self
    }

    pub fn pid(mut self, pid: impl Into<String>) -> Self {
        self.pid = Some(pid.into());

        self
    }

    pub fn crop(mut self, mode: CropMode) -> Self {
        self.crop = Some(mode);

        self
    }

    pub fn no_padding(mut self) -> Self {
        self.padding = Some(0);

        self
    }

    /// Builds the full thumbnail URL with query parameters.
    pub fn build(&self) -> Result<String, anyhow::Error> {
        Ok(format!(
            "{THUMBNAIL_URL}?{}",
            serde_urlencoded::to_string(self)?
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_url() {
        let url = UrlBuilder::new("OHR.Test_EN-US123_UHD.jpg")
            .build()
            .unwrap();

        assert_eq!(url, "https://www.bing.com/th?id=OHR.Test_EN-US123_UHD.jpg");
    }

    #[test]
    fn test_url_with_size() {
        let url = UrlBuilder::new("OHR.Test_EN-US123_UHD.jpg")
            .width(1920)
            .height(1080)
            .build()
            .unwrap();

        assert_eq!(
            url,
            "https://www.bing.com/th?id=OHR.Test_EN-US123_UHD.jpg&w=1920&h=1080"
        );
    }

    #[test]
    fn test_url_with_all_options() {
        let url = UrlBuilder::new("OHR.Test_EN-US123_UHD.jpg")
            .pid("hp")
            .width(200)
            .height(200)
            .no_padding()
            .crop(CropMode::SmartRatio)
            .build()
            .unwrap();

        assert_eq!(
            url,
            "https://www.bing.com/th?id=OHR.Test_EN-US123_UHD.jpg&w=200&h=200&pid=hp&p=0&c=7"
        );
    }
}
