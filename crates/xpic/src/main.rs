use crate::CLI::{Download, Export, List};
use anyhow::anyhow;
use clap::{Args, Parser};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use futures::StreamExt;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xpic::bing::Market;
use xpic::{fetch_image, list_images, Image, ListImagesRequestBuilder};

/// Bing wallpapers
#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
enum CLI {
    /// List Bing wallpapers
    List(QueryArgs),

    /// Download recent wallpapers to a directory
    Download {
        /// The output directory
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,

        #[command(flatten)]
        args: QueryArgs,
    },

    /// Export wallpaper metadata to JSON files
    Export {
        /// The output directory
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,
    },
}

#[derive(Args)]
struct QueryArgs {
    /// The number of wallpapers
    #[arg(short, long, default_value_t = 8)]
    number: usize,

    /// Index (0 = today)
    #[arg(short, long, default_value_t = 0)]
    index: usize,

    /// Market code
    #[arg(short, long)]
    market: Option<Market>,

    /// Ultra High Definition
    #[arg(short, long, default_value_t = true)]
    uhd: bool,
}

impl QueryArgs {
    fn into_builder(self) -> ListImagesRequestBuilder<'static> {
        list_images()
            .number(self.number)
            .index(self.index)
            .market_option(self.market)
            .uhd(self.uhd)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = CLI::parse();

    match cli {
        List(args) => {
            let images = args
                .into_builder()
                .send()
                .await
                .map_err(|err| anyhow!("failed to list wallpapers: {err}"))?;

            print_images_table(images);
        }
        Download { output, args } => {
            download_wallpapers(&output, args)
                .await
                .map_err(|err| anyhow!("failed to save wallpapers: {err}"))?;
        }
        Export { output } => {
            export_metadata(&output)
                .await
                .map_err(|err| anyhow!("failed to export metadata: {err}"))?;
        }
    }

    Ok(())
}

fn print_images_table(images: Vec<Image>) {
    let mut table = Table::new();

    table
        .load_preset(UTF8_BORDERS_ONLY)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Date").add_attribute(Attribute::Bold),
            Cell::new("Title").add_attribute(Attribute::Bold),
            Cell::new("Link").add_attribute(Attribute::Bold),
        ]);

    for image in images {
        table.add_row(vec![
            Cell::new(image.start_date).fg(Color::DarkYellow),
            Cell::new(image.title).fg(Color::DarkGreen),
            Cell::new(image.url).fg(Color::DarkCyan),
        ]);
    }

    println!("{table}");
}

async fn download_file(id: impl Into<String>, path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    if path.as_ref().exists() {
        return Ok(());
    }

    let resp = fetch_image(id).await?.error_for_status()?;

    let mut file = tokio::fs::File::create(path).await?;
    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;
    }

    Ok(())
}

async fn download_wallpapers(dir: impl AsRef<Path>, args: QueryArgs) -> Result<(), anyhow::Error> {
    let dir = dir.as_ref();

    tokio::fs::create_dir_all(dir).await?;

    let tasks = args
        .into_builder()
        .send()
        .await?
        .into_iter()
        .filter_map(|image| {
            let path = dir.join(&image.id);
            if path.exists() {
                return None;
            }

            Some(tokio::spawn(async move {
                let result = download_file(image.id, path)
                    .await
                    .map_err(|err| anyhow!("download failed: {err}"));

                if let Err(err) = result {
                    eprintln!("{err}");
                }
            }))
        });

    futures::future::join_all(tasks).await;
    Ok(())
}

async fn update_metadata_file(
    path: impl AsRef<Path>,
    mut images: Vec<Image>,
) -> Result<(), anyhow::Error> {
    let path = path.as_ref();

    if path.exists() {
        let file = tokio::fs::File::open(path).await?;
        let mut reader = tokio::io::BufReader::new(file);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;

        images.extend(serde_json::from_slice::<Vec<Image>>(&buffer)?);
    }

    images.sort_by(|a, b| a.hash.cmp(&b.hash));
    images.dedup_by(|a, b| a.hash == b.hash);

    images.sort_by(|a, b| b.start_date.cmp(&a.start_date));

    let mut data = serde_json::to_vec_pretty(&images)?;
    data.push(b'\n');
    tokio::fs::write(path, data).await?;

    Ok(())
}

async fn export_metadata(dir: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    let dir = dir.as_ref();

    tokio::fs::create_dir_all(dir).await?;

    // ROW: Rest of the World
    let mut market_images: HashMap<Market, Vec<Image>> = HashMap::new();

    for market in Market::iter() {
        let images = list_images().market(market).send().await?;

        for image in images {
            if let Some(id) = image.id_parsed.as_ref() {
                if let Some(market) = id.market {
                    market_images.entry(market).or_default().push(image);
                }
            }
        }
    }

    for (market, images) in market_images {
        let mut path = dir.join(market.code());
        path.set_extension("json");

        update_metadata_file(path, images).await?;
    }

    Ok(())
}
