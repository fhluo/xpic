use crate::bing::{Query, QueryParams, ThumbnailParams, ThumbnailQuery};
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
            query: Query::new(),
        }
    }

    pub async fn fetch_image(&self, id: impl Into<String>) -> reqwest::Result<reqwest::Response> {
        self.bing.thumbnail(&ThumbnailQuery::new(id)).await
    }

    pub fn fetch_thumbnail(&self, id: impl Into<String>) -> ThumbnailRequestBuilder<'_> {
        ThumbnailRequestBuilder {
            client: self,
            query: ThumbnailQuery::new(id),
        }
    }
}

pub struct ImagesRequestBuilder<'a> {
    client: &'a Client,
    query: Query,
}

impl QueryParams for ImagesRequestBuilder<'_> {
    fn query(&self) -> &Query {
        &self.query
    }

    fn query_mut(&mut self) -> &mut Query {
        &mut self.query
    }
}

impl ImagesRequestBuilder<'_> {
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
    query: ThumbnailQuery,
}

impl ThumbnailParams for ThumbnailRequestBuilder<'_> {
    fn query(&self) -> &ThumbnailQuery {
        &self.query
    }

    fn query_mut(&mut self) -> &mut ThumbnailQuery {
        &mut self.query
    }
}

impl ThumbnailRequestBuilder<'_> {
    pub async fn send(self) -> reqwest::Result<reqwest::Response> {
        self.client.bing.thumbnail(&self.query).await
    }
}
