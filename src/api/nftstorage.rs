use std::path::Path;

use eyre::Context;

static NFT_STORAGE_API_URL: &str = "https://api.nft.storage";

pub struct Client {
    client: reqwest::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn upload_car_to_nft_storage<P: AsRef<Path>>(
        &self,
        car_file_path: P,
    ) -> eyre::Result<()> {
        let car_file_path = car_file_path.as_ref();
        log::info!(
            "Uploading {} to NFT.Storage",
            car_file_path.to_string_lossy()
        );

        let car_file_contents = tokio::fs::read(car_file_path).await?;
        if car_file_contents.len() > 1000 * 1000 * 100 {
            // TODO: Split CAR file
            // See https://nft.storage/docs/concepts/car-files/#splitting-cars-for-upload-to-nftstorage
            eyre::bail!("Car file is too large");
        }

        let builder = self
            .client
            .post(format!("{}/upload", NFT_STORAGE_API_URL).as_str())
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/car")
            .body(car_file_contents);

        builder.send().await?.error_for_status().wrap_err(format!(
            "Failed to upload {} to NFT.Storage",
            car_file_path.display()
        ))?;

        Ok(())
    }
}
