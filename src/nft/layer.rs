use std::fs::DirEntry;
use std::path::PathBuf;

use crate::{Image, ImagePath, NftgenError};

/// Represents a value for single NFT layer
#[derive(Debug)]
pub struct Layer {
    pub image_path: PathBuf,
    pub weight: u32,
}

impl Layer {
    pub fn new<P: Into<PathBuf>>(image_path: P, weight: u32) -> Self {
        Layer {
            image_path: image_path.into(),
            weight,
        }
    }

    /// Returns none if the filename is not valid unicode
    pub fn name(&self) -> Option<&str> {
        self.image_path.file_stem()?.to_str()?.split('#').next()
    }

    /// Reads PNG from `self.image_path` and returns an `Image`
    pub fn get_image(&self) -> Result<Image, NftgenError> {
        Image::try_from(ImagePath(&self.image_path))
    }

    /// Parses weight from file stem of image file
    /// `filestem` - A stem of a file i.e. filename without extension e.g. red#5
    fn parse_weight_from_file_stem(filestem: &str) -> Result<u32, NftgenError> {
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
    type Error = NftgenError;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        if let Some(name) = entry.path().file_stem() {
            match name.to_str() {
                Some(name) => Ok(Layer::new(
                    &entry.path(),
                    Layer::parse_weight_from_file_stem(name)?,
                )),
                None => Err(NftgenError::InvalidFilename(entry.path())),
            }
        } else {
            Err(NftgenError::InvalidFilename(entry.path()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_str::assert_str_eq;

    use crate::nft::tests::fixture::Fixture;

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
        let fixture = Fixture::create_layers_dirs("minimal.png", &["background"]);
        let image_path = fixture.path.join("background/image1#1.png");

        let layer = Layer::new(&image_path, 5);
        let image = layer.get_image();
        assert!(image.is_ok());
    }

    #[test]
    fn layer_get_image_returns_err_for_invalid_png() {
        let fixture = Fixture::create_layers_dirs("empty.png", &["background"]);
        let image_path = fixture.path.join("background/image1#1.png");

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
