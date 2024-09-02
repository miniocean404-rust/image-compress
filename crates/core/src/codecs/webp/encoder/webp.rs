use std::io::{BufReader, Cursor};

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

use crate::codecs::webp::decoder::WebPDecoder;
use crate::codecs::webp::encoder::options::WebPOptions;


/// A WebP encoder
#[derive(Debug, Default)]
pub struct WebPEncoder {
    options: WebPOptions,
}

impl WebPEncoder {
    /// Create a new encoder
    pub fn new() -> WebPEncoder {
        WebPEncoder::default()
    }

    /// Create a new encoder with specified options
    pub fn new_with_options(options: WebPOptions) -> WebPEncoder {
        WebPEncoder { options }
    }

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);
        let reader = BufReader::new(cursor);

        let decoder = WebPDecoder::try_new(reader)?;

        let image = Image::from_decoder(decoder)?;

        let mut compress_buf = Cursor::new(vec![]);
        self.encode(&image, &mut compress_buf)?;

        Ok(compress_buf.into_inner())
    }
}

impl EncoderTrait for WebPEncoder {
    fn name(&self) -> &'static str {
        "webp"
    }

    fn encode_inner<T: ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        let options = webp::WebPConfig::from(self.options);
        let (width, height) = image.dimensions();

        let mut writer = ZWriter::new(sink);

        if image.is_animated() {
            let frames = image.flatten_to_u8();

            let mut encoder = webp::AnimEncoder::new(width as u32, height as u32, &options);

            encoder.set_bgcolor([0, 0, 0, 0]);
            encoder.set_loop_count(frames.len() as i32);

            frames.iter().try_for_each(|frame| {
                // TODO: add frame timestamp

                let frame = match image.colorspace() {
                    ColorSpace::RGB => {
                        webp::AnimFrame::from_rgb(frame, width as u32, height as u32, 500)
                    }
                    ColorSpace::RGBA => {
                        webp::AnimFrame::from_rgba(frame, width as u32, height as u32, 500)
                    }
                    cs => {
                        return Err(ImageErrors::EncodeErrors(
                            ImgEncodeErrors::UnsupportedColorspace(
                                cs,
                                self.supported_colorspaces(),
                            ),
                        ))
                    }
                };

                encoder.add_frame(frame);

                Ok(())
            })?;

            let res = encoder.encode();

            writer.write(&res).map_err(|e| {
                ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
            })?;

            Ok(writer.bytes_written())
        } else {
            let data = &image.flatten_to_u8()[0];

            let encoder = match image.colorspace() {
                ColorSpace::RGB => webp::Encoder::from_rgb(data, width as u32, height as u32),
                ColorSpace::RGBA => webp::Encoder::from_rgba(data, width as u32, height as u32),
                cs => {
                    return Err(ImageErrors::EncodeErrors(
                        ImgEncodeErrors::UnsupportedColorspace(cs, self.supported_colorspaces()),
                    ))
                }
            };

            let res = encoder.encode_advanced(&options).map_err(|e| {
                ImgEncodeErrors::ImageEncodeErrors(format!("webp encoding failed: {e:?}"))
            })?;

            writer.write(&res).map_err(|e| {
                ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
            })?;

            Ok(writer.bytes_written())
        }
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGB, ColorSpace::RGBA]
    }

    // TODO: update when new version with custom image format is released.
    fn format(&self) -> ImageFormat {
        ImageFormat::Unknown
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight]
    }

    fn default_depth(&self, _depth: BitDepth) -> BitDepth {
        BitDepth::Eight
    }

    fn supports_animated_images(&self) -> bool {
        true
    }
}
