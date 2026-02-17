pub mod bing;
mod client;
mod image;

use std::sync::LazyLock;

pub use crate::client::{Client, ImagesRequestBuilder, ThumbnailRequestBuilder};
pub use crate::image::{Copyright, Image, ID};

static DEFAULT_CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

/// Returns a reference to the global default [`Client`].
pub fn client() -> &'static Client {
    &DEFAULT_CLIENT
}

pub fn list_images() -> ImagesRequestBuilder<'static> {
    DEFAULT_CLIENT.list_images()
}

pub async fn fetch_image(id: impl Into<String>) -> reqwest::Result<reqwest::Response> {
    DEFAULT_CLIENT.fetch_image(id).await
}

pub fn fetch_thumbnail(id: impl Into<String>) -> ThumbnailRequestBuilder<'static> {
    DEFAULT_CLIENT.fetch_thumbnail(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bing::QueryParams;

    #[tokio::test]
    #[ignore]
    async fn test_list_images() {
        println!("{:#?}", list_images().number(1).send().await.unwrap())
    }
}
