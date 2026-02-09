use clap::Parser;
use std::path::{Path, PathBuf};
use xpic::bing_images;

/// Bing wallpapers
#[derive(Parser)]
#[command(version, about, arg_required_else_help(true))]
enum CLI {
    /// List Bing wallpapers
    List {
        /// The number of wallpapers to list
        #[arg(short)]
        number: Option<usize>,
    },
    /// Save wallpapers to a directory
    Save {
        /// The directory where wallpapers are saved
        dir: PathBuf,
    },
}

impl CLI {
    async fn run(self) {
        match self {
            CLI::List { number } => Self::list(number).await,
            CLI::Save { dir } => Self::save(dir).await,
        }
    }

    async fn list(number: Option<usize>) {
        match bing_images::get_images().await {
            Ok(images) => {
                if let Some(number) = number {
                    for url in images.into_iter().take(number) {
                        println!("{url}");
                    }
                } else {
                    for url in images {
                        println!("{url}");
                    }
                }
            }
            Err(err) => eprintln!("failed to get Bing wallpapers: {err}"),
        }
    }

    async fn save(dir: impl AsRef<Path>) {
        if let Err(err) = bing_images::copy_images_to(&dir).await {
            eprintln!(
                "failed to copy Bing wallpapers to {}:{}",
                dir.as_ref().display(),
                err
            );
        }
    }
}

#[tokio::main]
async fn main() {
    CLI::parse().run().await;
}
