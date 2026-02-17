mod client;
mod format;
mod market;
mod query;
mod response;
mod thumbnail_query;
mod url;

pub use client::Client;
pub use format::Format;
pub use market::Market;
pub use query::Query;
pub use response::{Image, Response, Tooltips};
pub use thumbnail_query::{CropMode, ThumbnailQuery};
pub use url::UrlBuilder;

use const_format::concatc;
use std::sync::LazyLock;

pub const BASE_URL: &str = "https://www.bing.com/";

pub const HP_IMAGE_ARCHIVE_URL: &str = concatc!(BASE_URL, "HPImageArchive.aspx");
pub const THUMBNAIL_URL: &str = concatc!(BASE_URL, "th");

pub static DEFAULT_CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

pub async fn hp_image_archive(query: &Query) -> reqwest::Result<Vec<Image>> {
    DEFAULT_CLIENT.hp_image_archive(query).await
}

pub async fn thumbnail(query: &ThumbnailQuery) -> reqwest::Result<reqwest::Response> {
    DEFAULT_CLIENT.thumbnail(query).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_hp_image_archive() {
        let query = Query::new().number(3);

        println!("{:#?}", hp_image_archive(&query).await.unwrap())
    }
}
