use clap::Parser;
use std::path::{Path, PathBuf};

mod spotlight;
mod util;

/// Windows Spotlight wallpapers
#[derive(Parser)]
enum CLI {
    /// List Windows Spotlight wallpapers
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
    fn run(self) {
        match self {
            CLI::List { number } => Self::list(number),
            CLI::Save { dir } => Self::save(dir),
        }
    }

    fn list(number: Option<usize>) {
        match spotlight::list_images() {
            Ok(images) => {
                if let Some(number) = number {
                    for path in images.into_iter().take(number) {
                        println!("{}", path.display());
                    }
                } else {
                    for path in images {
                        println!("{}", path.display());
                    }
                }
            }
            Err(err) => eprintln!("failed to get Windows Spotlight wallpapers: {err}"),
        }
    }

    fn save(dir: impl AsRef<Path>) {
        if let Err(err) = spotlight::copy_images_to(&dir) {
            eprintln!(
                "failed to copy Windows Spotlight wallpapers to {}:{}",
                dir.as_ref().display(),
                err
            );
        }
    }
}

fn main() {
    CLI::parse().run();
}
