use super::{Image, Query, Response, ThumbnailQuery, HP_IMAGE_ARCHIVE_URL, THUMBNAIL_URL};

pub struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    pub async fn hp_image_archive(&self, query: &Query) -> reqwest::Result<Vec<Image>> {
        Ok(self
            .http
            .get(HP_IMAGE_ARCHIVE_URL)
            .query(query)
            .send()
            .await?
            .error_for_status()?
            .json::<Response>()
            .await?
            .images)
    }

    pub async fn thumbnail(&self, query: &ThumbnailQuery) -> reqwest::Result<reqwest::Response> {
        self.http.get(THUMBNAIL_URL).query(query).send().await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}
