use crate::NftgenError;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

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

    pub fn stack(&mut self, images: &[Image]) {
        for (pixel_index, bottom_pixel) in self.data.chunks_mut(self.bytes_per_pixel).enumerate() {
            for image in images.iter().rev() {
                let top_pixel = &image.data[pixel_index * self.bytes_per_pixel
                    ..pixel_index * self.bytes_per_pixel + self.bytes_per_pixel];
                if top_pixel != &[0, 0, 0, 0] {
                    bottom_pixel[0] = top_pixel[0];
                    bottom_pixel[1] = top_pixel[1];
                    bottom_pixel[2] = top_pixel[2];
                    bottom_pixel[3] = top_pixel[3];
                    break;
                }
            }
        }
    }

    pub fn save<P: AsRef<Path>>(&self, output_path: P) -> Result<(), NftgenError> {
        let file = File::create(output_path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(self.data.as_slice())?;
        Ok(())
    }
}

pub struct ImagePath<P: AsRef<Path>>(pub P);

impl<P> TryFrom<ImagePath<P>> for Image
where
    P: AsRef<Path>,
{
    type Error = NftgenError;

    fn try_from(image_path: ImagePath<P>) -> Result<Self, Self::Error> {
        let decoder = png::Decoder::new(File::open(image_path.0)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;
        let info = reader.info();
        let bytes_per_pixel = info.bytes_per_pixel();
        let (width, height) = info.size();

        Ok(Image::new(buf, bytes_per_pixel, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nft::tests::fixture::Fixture;

    #[test]
    fn stack_works() {
        let fixture = Fixture::create_layers_dirs(
            "minimal.png",
            &["background", "eyes", "nose", "mouth", "hat"],
        );
        let bkg_image_path = fixture.path.join("background/image1#1.png");
        let eyes_image_path = fixture.path.join("eyes/image1#1.png");
        let nose_image_path = fixture.path.join("nose/image1#1.png");
        let mouth_image_path = fixture.path.join("mouth/image1#1.png");
        let hat_image_path = fixture.path.join("hat/image1#1.png");

        let mut bkg_image = Image::try_from(ImagePath(bkg_image_path)).unwrap();
        let eyes_image = Image::try_from(ImagePath(eyes_image_path)).unwrap();
        let nose_image = Image::try_from(ImagePath(nose_image_path)).unwrap();
        let mouth_image = Image::try_from(ImagePath(mouth_image_path)).unwrap();
        let hat_image = Image::try_from(ImagePath(hat_image_path)).unwrap();

        bkg_image.stack(&[eyes_image, nose_image, mouth_image, hat_image]);

        bkg_image.save("output.png").unwrap();
    }
}
