use std::path::PathBuf;
use thiserror::Error;

pub mod api;
pub mod cmd;

mod nft;
pub use nft::*;

#[derive(Error, Debug)]
pub enum NftgenError {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Encode(#[from] png::EncodingError),

    #[error(transparent)]
    Decode(#[from] png::DecodingError),

    #[error(transparent)]
    JsonEncode(#[from] serde_json::Error),

    #[error(transparent)]
    HttpRequestError(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidUtf8(#[from] std::str::Utf8Error),

    #[error("File name does not exist or is invalid unicode: '{0}'")]
    InvalidFilename(PathBuf),

    #[error("Directory path of layers is invalid: '{0}'")]
    InvalidLayerPath(PathBuf),

    #[error("Unknown layer: '{0}'")]
    UnknownLayer(String),

    #[error("CAR file > 100MB; too large to upload to nft.storage: '{0}'")]
    CarTooLarge(PathBuf),

    #[error("Could not run ipfs cli or parse output: '{0}'")]
    IpfsCommandError(String),
}
