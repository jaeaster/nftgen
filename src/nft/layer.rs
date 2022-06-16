use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use color_eyre::eyre;
use eyre::Context;
use image::DynamicImage;
use log;

/// Represents a value for single NFT layer
pub struct Layer {
    image_path: PathBuf,
    pub weight: u32,
}

impl Layer {
    pub fn new(image_path: &Path, weight: u32) -> Self {
        Layer {
            image_path: image_path.to_owned(),
            weight,
        }
    }

    /// Returns none if the filename is not valid unicode
    pub fn name(&self) -> Option<&str> {
        self.image_path.file_stem()?.to_str()?.split('#').next()
    }

    pub fn get_image(&self) -> eyre::Result<DynamicImage> {
        image::open(&self.image_path).wrap_err(format!(
            "Failed to open image: {}",
            self.image_path.to_str().unwrap_or_default()
        ))
    }

    /// Parses weight from file stem of image file
    /// `filestem` - A stem of a file i.e. filename without extension e.g. red#5
    fn parse_weight_from_file_stem(filestem: &str) -> eyre::Result<u32> {
        let weight_str = filestem.split_once('#').unwrap_or(("", "")).1;

        match weight_str.parse::<u32>() {
            Ok(weight) => Ok(weight),
            Err(_) => {
                log::warn!(
                    "Invalid weight for layer with filestem: {}. Using default weight of 1",
                    filestem
                );
                Ok(1)
            }
        }
    }
}

impl TryFrom<DirEntry> for Layer {
    type Error = eyre::Error;

    fn try_from(entry: DirEntry) -> eyre::Result<Self> {
        if let Some(name) = entry.path().file_stem() {
            match name.to_str() {
                Some(name) => Ok(Layer::new(
                    &entry.path(),
                    Layer::parse_weight_from_file_stem(name)?,
                )),
                None => eyre::bail!("Invalid layer name: {}", entry.path().display()),
            }
        } else {
            eyre::bail!("Invalid layer name: {}", entry.path().display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_str::assert_str_eq;
    use std::env::temp_dir;
    static PNG: [u8; 67] = [
        0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa, 0x0, 0x0, 0x0, 0xd, 0x49, 0x48, 0x44, 0x52,
        0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x8, 0x6, 0x0, 0x0, 0x0, 0x1f, 0x15, 0xc4, 0x89,
        0x0, 0x0, 0x0, 0xa, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x0, 0x1, 0x0, 0x0, 0x5, 0x0,
        0x1, 0xd, 0xa, 0x2d, 0xb4, 0x0, 0x0, 0x0, 0x0, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
        0x82,
    ];

    static INVALID_PNG: [u8; 0] = [];

    #[test]
    fn layer_name_works() {
        let image_path = std::path::Path::new("layers/background/red#5.png");
        let layer = Layer::new(image_path, 5);
        assert_str_eq!(layer.name().unwrap(), "red");
    }

    #[test]
    fn layer_name_works_without_delimiter() {
        let image_path = std::path::Path::new("layers/background/red.png");
        let layer = Layer::new(image_path, 5);
        assert_str_eq!(layer.name().unwrap(), "red");
    }

    #[test]
    fn layer_name_returns_none_if_not_utf8() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        let image_path = std::path::Path::new(os_str);

        let layer = Layer::new(image_path, 5);
        assert_eq!(layer.name(), None);
    }

    #[test]
    fn layer_get_image_works() {
        let tmp = temp_dir();
        let image_path = tmp.join("image.png");
        std::fs::write(&image_path, PNG).expect("Write to tmp should work in test");

        let layer = Layer::new(&image_path, 5);
        let image = layer.get_image();
        assert!(image.is_ok());
    }

    #[test]
    fn layer_get_image_returns_err_for_invalid_png() {
        let tmp = temp_dir();
        let image_path = tmp.join("image_err.png");
        std::fs::write(&image_path, INVALID_PNG).expect("Write to tmp should work in test");

        let layer = Layer::new(&image_path, 5);
        let image = layer.get_image();
        assert!(image.is_err());
    }

    #[test]
    fn parse_weight_from_filestem_works() {
        assert_eq!(Layer::parse_weight_from_file_stem("beauty#10").unwrap(), 10)
    }

    #[test]
    fn parse_weight_from_filestem_returns_1_if_invalid_filestem() {
        assert_eq!(Layer::parse_weight_from_file_stem("beauty").unwrap(), 1)
    }
}
