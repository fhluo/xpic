use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Crop mode for thumbnail images.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CropMode {
    /// Blind Ratio cropping.
    ///
    /// Crops based on simple aspect ratio rules:
    /// - If aspect ratio < requested ratio, crops from the bottom
    /// - If aspect ratio > requested ratio, crops from left and right
    #[serde(rename = "4")]
    BlindRatio = 4,

    /// Smart Ratio cropping.
    ///
    /// Crops from the center of the image's region of interest outward, maintaining aspect ratio.
    /// Falls back to Blind Ratio if the region of interest cannot be determined.
    #[serde(rename = "7")]
    SmartRatio = 7,
}

/// Query parameters for the Bing thumbnail endpoint.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailQuery {
    /// Image identifier.
    pub id: String,

    /// Page identifier.
    pub pid: Option<String>,

    /// Width in pixels.
    ///
    /// If only width is specified, Bing maintains the aspect ratio.
    #[serde(rename = "w")]
    pub width: Option<u32>,

    /// Height in pixels.
    ///
    /// If only height is specified, Bing maintains the aspect ratio.
    #[serde(rename = "h")]
    pub height: Option<u32>,

    /// Prevent white padding when the requested size exceeds the original.
    ///
    /// Set to `0` to prevent padding.
    #[serde(rename = "p")]
    pub padding: Option<u32>,

    /// Crop mode.
    #[serde(rename = "c")]
    pub crop: Option<CropMode>,
}

impl ThumbnailQuery {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pid: None,
            width: None,
            height: None,
            padding: None,
            crop: None,
        }
    }
}

/// Shared builder methods for types that contain thumbnail parameters.
pub trait ThumbnailParams: Sized {
    fn query_mut(&mut self) -> &mut ThumbnailQuery;

    fn pid(mut self, pid: impl Into<String>) -> Self {
        self.query_mut().pid = Some(pid.into());

        self
    }

    fn pid_option(mut self, pid: Option<impl Into<String>>) -> Self {
        self.query_mut().pid = pid.map(|p| p.into());

        self
    }

    fn width(mut self, width: u32) -> Self {
        self.query_mut().width = Some(width);

        self
    }

    fn width_option(mut self, width: Option<u32>) -> Self {
        self.query_mut().width = width;

        self
    }

    fn height(mut self, height: u32) -> Self {
        self.query_mut().height = Some(height);

        self
    }

    fn height_option(mut self, height: Option<u32>) -> Self {
        self.query_mut().height = height;

        self
    }

    /// Prevent white padding when the requested size exceeds the original.
    fn no_padding(mut self) -> Self {
        self.query_mut().padding = Some(0);

        self
    }

    fn padding_option(mut self, padding: Option<u32>) -> Self {
        self.query_mut().padding = padding;

        self
    }

    fn crop(mut self, mode: CropMode) -> Self {
        self.query_mut().crop = Some(mode);

        self
    }

    fn crop_option(mut self, mode: Option<CropMode>) -> Self {
        self.query_mut().crop = mode;

        self
    }
}

impl ThumbnailParams for ThumbnailQuery {
    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_minimal() {
        let query = ThumbnailQuery::new("OHR.Test_EN-US123_UHD.jpg");
        assert_eq!(
            serde_urlencoded::to_string(&query).unwrap(),
            "id=OHR.Test_EN-US123_UHD.jpg"
        );
    }

    #[test]
    fn test_encode_with_resize() {
        let query = ThumbnailQuery::new("OHR.Test_EN-US123_UHD.jpg")
            .pid("hp")
            .width(1920)
            .height(1080)
            .crop(CropMode::BlindRatio);

        assert_eq!(
            serde_urlencoded::to_string(&query).unwrap(),
            "id=OHR.Test_EN-US123_UHD.jpg&pid=hp&w=1920&h=1080&c=4"
        );
    }

    #[test]
    fn test_encode_smart_crop_no_padding() {
        let query = ThumbnailQuery::new("OHR.Test_EN-US123_UHD.jpg")
            .width(200)
            .height(200)
            .no_padding()
            .crop(CropMode::SmartRatio);

        assert_eq!(
            serde_urlencoded::to_string(&query).unwrap(),
            "id=OHR.Test_EN-US123_UHD.jpg&w=200&h=200&p=0&c=7"
        );
    }
}
