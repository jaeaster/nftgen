use crate::config;
use clap::Parser;
use color_eyre::eyre;
use std::{env, path::PathBuf};

/// Arguments for nftgen CLI tool
#[derive(Parser, Debug)]
#[clap(name="nftgen", author, version, about, long_about = None, args_override_self = true)]
pub struct NFTGenArgs {
    /// Number of NFTs to generate
    #[clap(short, long)]
    pub num: u32,

    /// path to root directory of NFT layers
    #[clap(short, long, value_hint = clap::ValueHint::DirPath)]
    pub layers_path: PathBuf,

    /// Order of NFT layers from back to front
    #[clap(
        long,
        multiple_values(true),
        use_value_delimiter(true),
        require_value_delimiter(true)
    )]
    pub layers_order: Vec<String>,

    /// Name of the collection
    #[clap(short, long)]
    pub collection_name: String,

    /// Description for the collection
    #[clap(short, long)]
    pub description: String,

    /// Base URI for assets in the collection
    #[clap(short, long)]
    pub base_uri: String,
}

pub fn parse_nftgen_args() -> eyre::Result<NFTGenArgs> {
    let mut config_args = config::args()?;
    let mut cli_args = env::args_os();
    if let Some(bin) = cli_args.next() {
        config_args.insert(0, bin);
    }
    config_args.extend(cli_args);

    Ok(NFTGenArgs::parse_from(config_args))
}
