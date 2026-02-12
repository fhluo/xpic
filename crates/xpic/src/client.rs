use crate::{bing, Image};

pub struct Client {
    bing: bing::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl Client {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            bing: bing::Client::new(http),
        }
    }

    /// Returns a reference to the underlying Bing API client.
    pub fn bing(&self) -> &bing::Client {
        &self.bing
    }

    pub fn list_images(&self) -> ImagesRequestBuilder<'_> {
        ImagesRequestBuilder {
            client: self,
            query: bing::Query::new(),
        }
    }

    pub async fn fetch_image(&self, id: impl Into<String>) -> reqwest::Result<reqwest::Response> {
        self.bing.thumbnail(&bing::ThumbnailQuery::new(id)).await
    }

    pub fn fetch_thumbnail(&self, id: impl Into<String>) -> ThumbnailRequestBuilder<'_> {
        ThumbnailRequestBuilder {
            client: self,
            query: bing::ThumbnailQuery::new(id),
        }
    }
}

pub struct ImagesRequestBuilder<'a> {
    client: &'a Client,
    query: bing::Query,
}

impl ImagesRequestBuilder<'_> {
    pub fn index(mut self, idx: usize) -> Self {
        self.query.index = idx;

        self
    }

    pub fn number(mut self, n: usize) -> Self {
        self.query.number = n;

        self
    }

    pub fn market(mut self, market: bing::Market) -> Self {
        self.query.market = Some(market);

        self
    }

    pub fn market_option(mut self, market: Option<bing::Market>) -> Self {
        self.query.market = market;

        self
    }

    pub fn uhd(mut self, uhd: bool) -> Self {
        self.query.uhd = Some(uhd);

        self
    }

    pub fn uhd_option(mut self, uhd: Option<bool>) -> Self {
        self.query.uhd = uhd;

        self
    }

    pub async fn send(self) -> Result<Vec<Image>, anyhow::Error> {
        Ok(self
            .client
            .bing
            .hp_image_archive(&self.query)
            .await?
            .into_iter()
            .filter_map(|raw| Image::parse(raw).ok())
            .collect())
    }
}

pub struct ThumbnailRequestBuilder<'a> {
    client: &'a Client,
    query: bing::ThumbnailQuery,
}

impl ThumbnailRequestBuilder<'_> {
    pub fn width(mut self, width: u32) -> Self {
        self.query.width = Some(width);

        self
    }

    pub fn width_option(mut self, width: Option<u32>) -> Self {
        self.query.width = width;

        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.query.height = Some(height);

        self
    }

    pub fn height_option(mut self, height: Option<u32>) -> Self {
        self.query.height = height;

        self
    }

    pub fn crop(mut self, mode: bing::CropMode) -> Self {
        self.query.crop = Some(mode);

        self
    }

    pub fn crop_option(mut self, crop_mode: Option<bing::CropMode>) -> Self {
        self.query.crop = crop_mode;

        self
    }

    pub fn no_padding(mut self) -> Self {
        self.query.padding = Some(0);

        self
    }

    pub fn padding_option(mut self, padding: Option<u32>) -> Self {
        self.query.padding = padding;

        self
    }

    pub async fn send(self) -> reqwest::Result<reqwest::Response> {
        self.client.bing.thumbnail(&self.query).await
    }
}
