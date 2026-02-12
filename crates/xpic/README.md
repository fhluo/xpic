# Xpic

List and download Bing wallpapers.

## CLI

```shell
cargo install xpic
```

### List Wallpapers

```shell
xpic list
xpic list -n 3 -m en-US
```

### Download Wallpapers

```shell
xpic download -o ./wallpapers
xpic download -o ./wallpapers -n 3 -m zh-CN
```

## Library

Add `xpic` to your `Cargo.toml`:

```toml
[dependencies]
xpic = "0.3"
```

### List Wallpapers

```rust
use xpic::list_images;
use xpic::bing::Market;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let images = list_images()
        .number(8)
        .market(Market::EN_US)
        .send()
        .await?;

    for image in images {
        println!("{} - {}", image.title, image.url);
    }
    Ok(())
}
```

### Fetch Images

```rust
use xpic::{fetch_image, fetch_thumbnail};
use xpic::bing::CropMode;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let images = xpic::list_images().send().await?;
    let id = &images[0].id;

    // Fetch the original image
    let response = fetch_image(id).await?;

    // Fetch a resized thumbnail
    let response = fetch_thumbnail(id)
        .width(1920)
        .height(1080)
        .send()
        .await?;

    // Fetch with smart crop
    let response = fetch_thumbnail(id)
        .width(800)
        .height(600)
        .crop(CropMode::SmartRatio)
        .no_padding()
        .send()
        .await?;

    Ok(())
}
```
