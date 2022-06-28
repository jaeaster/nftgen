use crate::NftgenError;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    data: Vec<u8>,
    bytes_per_pixel: usize,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(data: Vec<u8>, bytes_per_pixel: usize, width: u32, height: u32) -> Self {
        Image {
            data,
            bytes_per_pixel,
            width,
            height,
        }
    }

    pub fn read<P: AsRef<Path>>(image_path: P) -> Result<Self, NftgenError> {
        let decoder = png::Decoder::new(File::open(image_path)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;
        let info = reader.info();
        let bytes_per_pixel = info.bytes_per_pixel();
        let (width, height) = info.size();

        Ok(Image::new(buf, bytes_per_pixel, width, height))
    }

    pub fn stack(&mut self, images: &[Image]) {
        for (pixel_index, bottom_pixel) in self.data.chunks_mut(self.bytes_per_pixel).enumerate() {
            for image in images.iter().rev() {
                let top_pixel = &image.data[pixel_index * self.bytes_per_pixel
                    ..pixel_index * self.bytes_per_pixel + self.bytes_per_pixel];

                if top_pixel.iter().any(|&p| p == 1) {
                    for (i, _) in top_pixel.iter().enumerate() {
                        bottom_pixel[i] = top_pixel[i];
                    }
                    break;
                }
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, output_path: P) -> Result<(), NftgenError> {
        let file = File::create(output_path)?;
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.data.as_slice())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nft::tests::fixture::Fixture;

    mod stack {
        use super::*;

        #[test]
        fn transparent_layers() {
            let image = Image::new(vec![0, 0, 0, 0], 4, 1, 1);
            let mut bkg_image = image.clone();
            let eyes_image = image.clone();

            bkg_image.stack(&[eyes_image]);
            assert_eq!(image.data, bkg_image.data);
        }

        #[test]
        fn layers_with_content() {
            let mut bkg_image = Image::new(vec![1, 0, 0, 1], 4, 1, 1);
            let eyes_image = Image::new(vec![0, 1, 1, 0], 4, 1, 1);

            bkg_image.stack(&[eyes_image]);
            assert_eq!(bkg_image.data, vec![0, 1, 1, 0]);
        }

        #[test]
        fn layers_with_two_pixels() {
            let mut bkg_image = Image::new(vec![1, 0, 0, 1, 1, 1, 1, 1], 4, 1, 1);
            let eyes_image = Image::new(vec![0, 0, 0, 0, 1, 0, 0, 0], 4, 1, 1);

            bkg_image.stack(&[eyes_image]);
            assert_eq!(bkg_image.data, vec![1, 0, 0, 1, 1, 0, 0, 0]);
        }

        #[test]
        fn three_layers() {
            let mut bkg_image = Image::new(vec![0, 0, 1], 1, 1, 1);
            let eyes_image = Image::new(vec![1, 0, 0], 1, 1, 1);
            let mouth_image = Image::new(vec![0, 1, 0], 1, 1, 1);

            bkg_image.stack(&[eyes_image, mouth_image]);
            assert_eq!(bkg_image.data, vec![1, 1, 1]);
        }
    }

    mod read {
        use super::*;

        #[test]
        fn transparent_image() {
            let fixture = Fixture::create_layers_dirs("minimal.png", &["background"]);
            let bkg_image_path = fixture.path.join("background/image1#1.png");

            let bkg_image = Image::read(bkg_image_path).unwrap();
            assert_eq!(bkg_image.data, vec![0, 0, 0, 0]);
            assert_eq!(bkg_image.bytes_per_pixel, 4);
            assert_eq!(bkg_image.width, 1);
            assert_eq!(bkg_image.height, 1);
        }

        #[test]
        fn empty_image() {
            let fixture = Fixture::create_layers_dirs("empty.png", &["background"]);
            let bkg_image_path = fixture.path.join("background/image1#1.png");

            let image_result = Image::read(bkg_image_path);
            assert!(matches!(image_result, Err(NftgenError::Decode(_))));
        }
    }

    #[test]
    fn save() {
        let fixture = Fixture::create_layers_dirs("minimal.png", &["background"]);
        let bkg_image_path = fixture.path.join("background/image1#1.png");

        let bkg_image = Image::read(bkg_image_path).unwrap();

        bkg_image.save(fixture.path.join("output.png")).unwrap();
    }
}
