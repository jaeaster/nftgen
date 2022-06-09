use clap::{Parser, Subcommand};
use std::env;

use crate::cmd::{config, generate::GenerateArgs, upload::UploadArgs};

/// Generate images and metadata for NFTs by layering PNGs together.
#[derive(Debug, Parser)]
#[clap(name = "nftgen", author, version, about, long_about = None, args_override_self = true)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(about = "Generate and upload nft images and metadata")]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(visible_alias = "g")]
    #[clap(about = "Generate nft images and metadata")]
    Generate(GenerateArgs),

    #[clap(about = "Upload nft images and metadata to IPFS")]
    #[clap(visible_alias = "u")]
    Upload(UploadArgs),
}

impl Opts {
    pub fn parse_from_config_and_cli() -> eyre::Result<Self> {
        let mut config_args = config::args()?;
        let mut cli_args = env::args_os();
        if let Some(bin) = cli_args.next() {
            config_args.insert(0, bin);
            if let Some(sub) = cli_args.next() {
                config_args.insert(1, sub);
            }
        }
        config_args.extend(cli_args);

        Ok(Opts::parse_from(config_args))
    }
}
