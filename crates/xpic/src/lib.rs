pub mod bing;
mod client;
mod image;

use std::sync::LazyLock;

pub use crate::client::{Client, ListImagesRequestBuilder};
pub use crate::image::{Copyright, Image, ID};

static DEFAULT_CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

/// Returns a reference to the global default [`Client`].
pub fn client() -> &'static Client {
    &DEFAULT_CLIENT
}

pub fn list_images() -> ListImagesRequestBuilder<'static> {
    DEFAULT_CLIENT.list_images()
}

pub async fn fetch_image(id: impl Into<String>) -> reqwest::Result<reqwest::Response> {
    DEFAULT_CLIENT.fetch_image(id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_list_images() {
        println!("{:#?}", list_images().number(1).send().await.unwrap())
    }
}
