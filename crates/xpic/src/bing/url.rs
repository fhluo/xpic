use super::{ThumbnailParams, ThumbnailQuery, THUMBNAIL_URL};

/// Builder for constructing Bing thumbnail URLs without making requests.
#[derive(Debug, Clone)]
pub struct UrlBuilder {
    query: ThumbnailQuery,
}

impl UrlBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            query: ThumbnailQuery::new(id),
        }
    }

    /// Builds the full thumbnail URL with query parameters.
    pub fn build(&self) -> Result<String, anyhow::Error> {
        Ok(format!(
            "{THUMBNAIL_URL}?{}",
            serde_urlencoded::to_string(&self.query)?
        ))
    }
}

impl ThumbnailParams for UrlBuilder {
    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        &mut self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bing::CropMode;

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
            "https://www.bing.com/th?id=OHR.Test_EN-US123_UHD.jpg&pid=hp&w=200&h=200&p=0&c=7"
        );
    }
}
