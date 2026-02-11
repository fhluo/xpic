mod format;
mod market;
mod query;
mod response;
mod thumbnail_query;

use const_format::concatc;
pub use format::Format;
pub use market::Market;
pub use query::Query;
pub use response::*;
use std::sync::LazyLock;
pub use thumbnail_query::{CropMode, ThumbnailQuery};

pub const BASE_URL: &str = "https://www.bing.com/";

const HP_IMAGE_ARCHIVE_URL: &str = concatc!(BASE_URL, "HPImageArchive.aspx");
const THUMBNAIL_URL: &str = concatc!(BASE_URL, "th");

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub async fn hp_image_archive(query: &Query) -> reqwest::Result<Vec<Image>> {
    Ok(CLIENT
        .get(HP_IMAGE_ARCHIVE_URL)
        .query(query)
        .send()
        .await?
        .error_for_status()?
        .json::<Response>()
        .await?
        .images)
}

pub async fn thumbnail(query: &ThumbnailQuery) -> reqwest::Result<reqwest::Response> {
    CLIENT
        .get(THUMBNAIL_URL)
        .query(query)
        .send()
        .await?
        .error_for_status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_list_images() {
        let query = Query::new().number(3);

        println!("{:#?}", hp_image_archive(&query).await.unwrap())
    }
}
