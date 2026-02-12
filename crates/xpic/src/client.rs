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

    pub fn list_images(&self) -> ListImagesRequestBuilder<'_> {
        ListImagesRequestBuilder {
            client: self,
            query: bing::Query::new(),
        }
    }
}

pub struct ListImagesRequestBuilder<'a> {
    client: &'a Client,
    query: bing::Query,
}

impl ListImagesRequestBuilder<'_> {
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
