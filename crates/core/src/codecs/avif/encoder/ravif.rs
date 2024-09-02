use std::io::{BufReader, Cursor};

use ravif::Img;
use rgb::FromSlice;
use zune_core::{
    bit_depth::BitDepth,
    bytestream::{ZByteWriterTrait, ZWriter},
    colorspace::ColorSpace,
};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

use crate::codecs::avif::decoder::AvifDecoder;

use super::options::AvifOptions;

/// A AVIF encoder
#[derive(Default)]
pub struct AvifEncoder {
    options: AvifOptions,
}

impl Default for AvifOptions {
    fn default() -> Self {
        Self {
            quality: 50.,
            alpha_quality: None,
            speed: 5,
            color_space: ravif::ColorSpace::YCbCr,
            alpha_color_mode: ravif::AlphaColorMode::UnassociatedClean,
        }
    }
}

impl AvifEncoder {
    /// Create a new encoder
    pub fn new() -> AvifEncoder {
        AvifEncoder::default()
    }

    /// Create a new encoder with specified options
    pub fn new_with_options(options: AvifOptions) -> AvifEncoder {
        AvifEncoder { options }
    }

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);
        let reader = BufReader::new(cursor);

        let decoder = AvifDecoder::try_new(reader)?;
        let image = Image::from_decoder(decoder)?;

        let mut compress_buf = Cursor::new(vec![]);
        self.encode(&image, &mut compress_buf)?;

        Ok(compress_buf.into_inner())
    }
}

impl EncoderTrait for AvifEncoder {
    fn name(&self) -> &'static str {
        "avif"
    }

    fn encode_inner<T: ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        let (width, height) = image.dimensions();
        let data = &image.flatten_to_u8()[0];

        let mut writer = ZWriter::new(sink);

        let encoder = ravif::Encoder::new()
            .with_quality(self.options.quality)
            .with_alpha_quality(self.options.alpha_quality.unwrap_or(self.options.quality))
            .with_speed(self.options.speed)
            .with_internal_color_space(self.options.color_space)
            .with_alpha_color_mode(self.options.alpha_color_mode);

        match image.colorspace() {
            ColorSpace::RGB => {
                let img = Img::new(data.as_slice().as_rgb(), width, height);
                let result = encoder
                    .encode_rgb(img)
                    .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

                writer.write(&result.avif_file).map_err(|e| {
                    ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
                })?;

                Ok(writer.bytes_written())
            }
            ColorSpace::RGBA => {
                let img = Img::new(data.as_slice().as_rgba(), width, height);
                let result = encoder
                    .encode_rgba(img)
                    .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

                writer.write(&result.avif_file).map_err(|e| {
                    ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
                })?;

                Ok(writer.bytes_written())
            }
            cs => Err(ImageErrors::EncodeErrors(
                ImgEncodeErrors::UnsupportedColorspace(cs, self.supported_colorspaces()),
            )),
        }
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGB, ColorSpace::RGBA]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Unknown
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight]
    }

    fn default_depth(&self, _depth: BitDepth) -> BitDepth {
        BitDepth::Eight
    }
}
