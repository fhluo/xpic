mod format;
mod market;
mod query;
mod response;

pub use format::Format;
pub use market::Market;
pub use query::Query;
pub use response::*;

use std::sync::LazyLock;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub async fn list_images(query: &Query) -> reqwest::Result<Vec<Image>> {
    Ok(CLIENT
        .get("https://www.bing.com/HPImageArchive.aspx")
        .query(query)
        .send()
        .await?
        .error_for_status()?
        .json::<Response>()
        .await?
        .images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_list_images() {
        let query = Query::new().number(3);

        println!("{:#?}", list_images(&query).await.unwrap())
    }
}
