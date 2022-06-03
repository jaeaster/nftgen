use color_eyre::eyre;
use std::{fs::DirEntry, path::PathBuf};

use clap::Parser;
use nftgen::{self, LayerGroup};

/// Arguments for nftgen CLI tool
#[derive(Parser, Debug)]
#[clap(name="nftgen", author, version, about, long_about = None)]
struct Args {
    /// Number of NFTs to generate
    #[clap(short, long)]
    num: u32,

    /// path to root directory of NFT layers
    #[clap(short, long, value_hint = clap::ValueHint::DirPath)]
    layers_path: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let nft_count = args.num;
    let layers_path = args.layers_path;

    let layer_dirs: Vec<PathBuf> = match layers_path.read_dir() {
        Ok(layers) => layers
            .collect::<Result<Vec<DirEntry>, _>>()?
            .into_iter()
            .map(|l| l.path())
            .filter(|l| l.is_dir())
            .collect(),
        Err(_) => eyre::bail!(
            "Failed to read layers directory: {}",
            layers_path.to_string_lossy()
        ),
    };

    let layer_groups = layer_dirs
        .into_iter()
        .map(|layer_dir| LayerGroup::new(layer_dir.as_path()))
        .collect::<Result<Vec<_>, _>>()?;

    for n in 0..nft_count {
        println!("\nNFT # {}", n);
        for layer_group in layer_groups.iter() {
            let layer = layer_group.pick();
            println!("{}", layer.name);
        }
    }

    Ok(())
}
