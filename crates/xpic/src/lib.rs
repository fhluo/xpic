pub mod bing;
mod image;

pub use crate::image::*;
pub use bing::Query;

pub async fn list_images(query: &Query) -> Result<Vec<Image>, anyhow::Error> {
    Ok(bing::list_images(query)
        .await?
        .into_iter()
        .filter_map(|raw| Image::parse(raw).ok())
        .collect::<Vec<Image>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_list_images() {
        let query = Query::new().number(1);

        println!("{:#?}", list_images(&query).await.unwrap())
    }
}
