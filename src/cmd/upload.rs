use std::process::{Output, Stdio};

use clap::Parser;
use eyre::Context;
use futures::future::BoxFuture;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

use crate::cmd::Cmd;

#[derive(Debug, Clone, Parser)]
pub struct UploadArgs {
    /// path to root directory of NFT layers
    #[clap(short, long, default_value="./nftgen-output", value_hint = clap::ValueHint::DirPath)]
    pub output_path: PathBuf,

    /// Base URI for assets in the collection
    #[clap(short, long)]
    pub base_uri: String,

    /// API Key for NFT.Storage
    #[clap(long)]
    pub api_key: String,
}

impl Cmd for UploadArgs {
    type Output = BoxFuture<'static, eyre::Result<()>>;

    fn run(self) -> eyre::Result<Self::Output> {
        let UploadArgs {
            output_path,
            base_uri,
            api_key,
        } = self;
        Ok(Box::pin(Self::upload(output_path, base_uri, api_key)))
    }
}

impl UploadArgs {
    async fn upload(output_path: PathBuf, base_uri: String, api_key: String) -> eyre::Result<()> {
        let images_path = output_path.as_path().join("images");
        let car_file_path = output_path.as_path().join("images.car");
        let metadata_path = output_path.as_path().join("metadata");

        log::info!("Running `ipfs init`");
        run_ipfs_init().await?;

        // log::info!("Starting `ipfs daemon`");
        // let mut ipfs_daemon = run_ipfs_daemon()?;
        // block_until_ipfs_daemon_ready(&mut ipfs_daemon).await?;
        // log::info!("Started `ipfs daemon`");

        log::info!("Running `ipfs add -r {}`", images_path.display());
        let ipfs_add_output =
            run_ipfs_add(images_path.as_os_str().to_string_lossy().to_string()).await?;

        let cid = parse_cid_from_ipfs_add_output(&ipfs_add_output)?;

        log::info!("Running `ipfs dag export {}`", cid);
        run_ipfs_dag_export(&cid, &car_file_path).await?;

        log::info!("Uploading images to NFT.Storage");
        upload_images_to_nft_storage(&car_file_path, &api_key).await?;

        // log::info!("Updating metadata with image directory CID");
        // update_metadata(&metadata_path, &cid).await?;

        // log::info!("Uploading metadata to NFT.Storage");
        // upload_metadata_to_nft_storage(&car_file_path, &api_key).await?;

        Ok(())
    }
}

async fn run_ipfs_init() -> eyre::Result<Output> {
    Command::new("ipfs")
        .args(&["init"])
        .output()
        .await
        .wrap_err("Failed to run `ipfs init`")
}

fn run_ipfs_daemon() -> eyre::Result<Child> {
    Command::new("ipfs")
        .args(&["daemon"])
        .stdout(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .wrap_err("Daemon failed to run")
}

async fn block_until_ipfs_daemon_ready(ipfs_daemon: &mut Child) -> eyre::Result<()> {
    let stdout = ipfs_daemon.stdout.take().unwrap();
    let mut stdout_reader = BufReader::new(stdout).lines();

    while let Some(line) = stdout_reader.next_line().await? {
        if line.contains("Daemon is ready") {
            return Ok(());
        }
    }

    Err(eyre::eyre!("Failed to read from ipfs daemon stdout"))
}

async fn run_ipfs_add(output_path: String) -> eyre::Result<Output> {
    Command::new("ipfs")
        .args(&["add", "-r", &output_path])
        .output()
        .await
        .wrap_err("Failed to run `ipfs add`")
}

fn parse_cid_from_ipfs_add_output(raw_output: &Output) -> eyre::Result<String> {
    let output = std::str::from_utf8(raw_output.stdout.as_slice())?;

    let mut lines = output.lines();
    if let Some(last_line) = lines.nth_back(0) {
        if let Some(cid) = last_line.split_whitespace().nth(1) {
            Ok(cid.to_string())
        } else {
            Err(eyre::eyre!("Failed to parse cid from ipfs add output"))
        }
    } else {
        Err(eyre::eyre!("Failed to parse cid from ipfs add output"))
    }
}

async fn run_ipfs_dag_export(cid: &str, car_file_path: &Path) -> eyre::Result<()> {
    let output = Command::new("ipfs")
        .args(&["dag", "export", cid])
        .output()
        .await
        .wrap_err("Failed to run `ipfs dag export`")?;

    tokio::fs::write(car_file_path, output.stdout)
        .await
        .wrap_err(format!(
            "failed to write car file to {}",
            car_file_path.display()
        ))
}

async fn upload_images_to_nft_storage(car_file_path: &Path, api_key: &str) -> eyre::Result<()> {
    let car_file_contents = tokio::fs::read(car_file_path).await?;
    let client = Client::new();
    let builder = client
        .post("https://api.nft.storage/upload")
        .bearer_auth(api_key)
        .header("Content-Type", "application/car")
        .body(car_file_contents);

    builder
        .send()
        .await?
        .error_for_status()
        .wrap_err("Failed to upload images to NFT.Storage")?;

    Ok(())
}

// {
//     Ok(response) => {
//         log::info!("Uploaded images to NFT.Storage");
//         log::info!("Response: {:?}", response);
//     }
//     Err(e) => {
//         log::error!("Error: {:?}", e);
//     }
// }
