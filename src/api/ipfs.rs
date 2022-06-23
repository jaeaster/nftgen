use std::path::Path;
use std::process::{Output, Stdio};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

use crate::NftgenError;

pub struct Cli {
    daemon: Option<Child>,
}

impl Cli {
    pub async fn new() -> Result<Self, NftgenError> {
        Cli::init().await?;

        Ok(Self { daemon: None })
    }

    pub async fn add(&self, output_path: &str) -> Result<String, NftgenError> {
        log::info!("Running `ipfs add -r {}`", output_path);
        let add_output = Command::new("ipfs")
            .args(&["add", "-r", output_path])
            .output()
            .await?;

        Cli::parse_cid_from_ipfs_add_output(&add_output)
    }

    pub async fn dag_export<P: AsRef<Path>>(
        &self,
        cid: &str,
        car_file_path: P,
    ) -> Result<(), NftgenError> {
        let car_file_path = car_file_path.as_ref();
        log::info!("Running `ipfs dag export {}`", cid);
        let output = Command::new("ipfs")
            .args(&["dag", "export", cid])
            .output()
            .await?;

        Ok(tokio::fs::write(car_file_path, output.stdout).await?)
    }

    pub async fn daemon_and_block_until_ready(&mut self) -> Result<(), NftgenError> {
        log::info!("Starting `ipfs daemon`");

        self.daemon = Some(
            Command::new("ipfs")
                .args(&["daemon"])
                .stdout(Stdio::piped())
                .kill_on_drop(true)
                .spawn()?,
        );

        self.block_until_ipfs_daemon_ready().await?;
        log::info!("Started `ipfs daemon`");

        Ok(())
    }

    async fn init() -> Result<Output, NftgenError> {
        log::info!("Running `ipfs init`");
        Ok(Command::new("ipfs").args(&["init"]).output().await?)
    }

    fn parse_cid_from_ipfs_add_output(raw_output: &Output) -> Result<String, NftgenError> {
        let output = std::str::from_utf8(raw_output.stdout.as_slice())?;

        let mut lines = output.lines();
        if let Some(last_line) = lines.nth_back(0) {
            if let Some(cid) = last_line.split_whitespace().nth(1) {
                Ok(cid.to_string())
            } else {
                Err(NftgenError::IpfsCommandError("ipfs add".to_string()))
            }
        } else {
            Err(NftgenError::IpfsCommandError("ipfs add".to_string()))
        }
    }

    async fn block_until_ipfs_daemon_ready(&mut self) -> Result<(), NftgenError> {
        if let Some(daemon) = &mut self.daemon {
            let stdout = daemon.stdout.take().unwrap();
            let mut stdout_reader = BufReader::new(stdout).lines();

            while let Some(line) = stdout_reader.next_line().await? {
                if line.contains("Daemon is ready") {
                    return Ok(());
                }
            }

            Err(NftgenError::IpfsCommandError("ipfs daemon".to_string()))
        } else {
            Err(NftgenError::IpfsCommandError("ipfs daemon".to_string()))
        }
    }
}
