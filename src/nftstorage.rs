use std::path::Path;

use eyre::Context;
use reqwest::Client;

pub async fn upload_images_to_nft_storage(car_file_path: &Path, api_key: &str) -> eyre::Result<()> {
    let car_file_contents = tokio::fs::read(car_file_path).await?;
    if car_file_contents.len() > 1000 * 1000 * 100 {
        // TODO: Split CAR file
        // See https://nft.storage/docs/concepts/car-files/#splitting-cars-for-upload-to-nftstorage
        eyre::bail!("Car file is too large");
    }

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
