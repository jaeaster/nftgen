use std::path::Path;
use std::process::{Output, Stdio};

use eyre::Context;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

pub struct Cli {
    daemon: Option<Child>,
}

impl Cli {
    pub async fn new() -> eyre::Result<Self> {
        Cli::init().await?;

        Ok(Self { daemon: None })
    }

    pub async fn add(&self, output_path: &str) -> eyre::Result<String> {
        log::info!("Running `ipfs add -r {}`", output_path);
        let add_output = Command::new("ipfs")
            .args(&["add", "-r", output_path])
            .output()
            .await
            .wrap_err("Failed to run `ipfs add`")?;

        Cli::parse_cid_from_ipfs_add_output(&add_output)
    }

    pub async fn dag_export<P: AsRef<Path>>(
        &self,
        cid: &str,
        car_file_path: P,
    ) -> eyre::Result<()> {
        let car_file_path = car_file_path.as_ref();
        log::info!("Running `ipfs dag export {}`", cid);
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

    pub async fn daemon_and_block_until_ready(&mut self) -> eyre::Result<()> {
        log::info!("Starting `ipfs daemon`");

        self.daemon = Some(
            Command::new("ipfs")
                .args(&["daemon"])
                .stdout(Stdio::piped())
                .kill_on_drop(true)
                .spawn()
                .wrap_err("Daemon failed to run")?,
        );

        self.block_until_ipfs_daemon_ready().await?;
        log::info!("Started `ipfs daemon`");

        Ok(())
    }

    async fn init() -> eyre::Result<Output> {
        log::info!("Running `ipfs init`");
        Command::new("ipfs")
            .args(&["init"])
            .output()
            .await
            .wrap_err("Failed to run `ipfs init`")
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

    async fn block_until_ipfs_daemon_ready(&mut self) -> eyre::Result<()> {
        if let Some(daemon) = &mut self.daemon {
            let stdout = daemon.stdout.take().unwrap();
            let mut stdout_reader = BufReader::new(stdout).lines();

            while let Some(line) = stdout_reader.next_line().await? {
                if line.contains("Daemon is ready") {
                    return Ok(());
                }
            }

            Err(eyre::eyre!("Failed to read from ipfs daemon stdout"))
        } else {
            Err(eyre::eyre!("ipfs daemon not running"))
        }
    }
}
