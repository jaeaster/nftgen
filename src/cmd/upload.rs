use clap::Parser;
use futures::future::BoxFuture;
use std::path::PathBuf;

use crate::api::ipfs;
use crate::api::nftstorage;
use crate::cmd::Cmd;
use crate::nft::MetadataWriter;

#[derive(Debug, Clone, Parser)]
pub struct UploadArgs {
    /// path to the output directory of NFT images and metadata
    #[clap(short, long, default_value="./nftgen-output", value_hint = clap::ValueHint::DirPath)]
    pub output_path: PathBuf,

    /// API Key for NFT.Storage
    #[clap(long)]
    pub api_key: String,
}

impl Cmd for UploadArgs {
    type Output = BoxFuture<'static, eyre::Result<()>>;

    fn run(self) -> eyre::Result<Self::Output> {
        let UploadArgs {
            output_path,
            api_key,
        } = self;
        Ok(Box::pin(Self::upload(output_path, api_key)))
    }
}

impl UploadArgs {
    async fn upload(output_path: PathBuf, api_key: String) -> eyre::Result<()> {
        let images_path = output_path.as_path().join("images");
        let metadata_path = output_path.as_path().join("metadata");
        let images_car_file_path = output_path.as_path().join("images.car");
        let metadata_car_file_path = output_path.as_path().join("metadata.car");

        let ipfs_cli = ipfs::Cli::new().await?;

        let images_cid = ipfs_cli
            .add(images_path.as_os_str().to_string_lossy().as_ref())
            .await?;

        MetadataWriter::new(metadata_path.as_path()).update_base_uri_for_all_images(&images_cid)?;

        let metadata_cid = ipfs_cli
            .add(metadata_path.as_os_str().to_string_lossy().as_ref())
            .await?;

        ipfs_cli
            .dag_export(&images_cid, &images_car_file_path)
            .await?;

        ipfs_cli
            .dag_export(&metadata_cid, &metadata_car_file_path)
            .await?;

        let nftstorage_client = nftstorage::Client::new(api_key);
        nftstorage_client
            .upload_car_to_nft_storage(&images_car_file_path)
            .await?;

        nftstorage_client
            .upload_car_to_nft_storage(&metadata_car_file_path)
            .await?;

        Ok(())
    }
}
