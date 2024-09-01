use std::io::{BufReader, Cursor};

use libwebp_sys::WebPImageHint;
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

use crate::codecs::OptionsTrait;

use super::decoder::WebPDecoder;

impl From<WebPOptions> for webp::WebPConfig {
    fn from(value: WebPOptions) -> Self {
        let mut config = webp::WebPConfig::new().unwrap();

        config.lossless = value.lossless;
        config.quality = value.quality;
        config.method = value.method;
        config.image_hint = value.image_hint;
        config.target_size = value.target_size;
        config.target_PSNR = value.target_PSNR;
        config.segments = value.segments;
        config.sns_strength = value.sns_strength;
        config.filter_strength = value.filter_strength;
        config.filter_sharpness = value.filter_sharpness;
        config.filter_type = value.filter_type;
        config.autofilter = value.autofilter;
        config.alpha_compression = value.alpha_compression;
        config.alpha_filtering = value.alpha_filtering;
        config.alpha_quality = value.alpha_quality;
        config.pass = value.pass;
        config.show_compressed = value.show_compressed;
        config.preprocessing = value.preprocessing;
        config.partitions = value.partitions;
        config.partition_limit = value.partition_limit;
        config.emulate_jpeg_size = value.emulate_jpeg_size;
        config.thread_level = value.thread_level;
        config.low_memory = value.low_memory;
        config.near_lossless = value.near_lossless;
        config.exact = value.exact;
        config.use_delta_palette = value.use_delta_palette;
        config.use_sharp_yuv = value.use_sharp_yuv;
        config.qmin = value.qmin;
        config.qmax = value.qmax;

        config
    }
}

// impl OptionsTrait for WebPOptions {}

// impl Default for WebPOptions {}

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct WebPOptions {
    pub lossless: i32,
    pub quality: f32,
    pub method: i32,
    pub image_hint: WebPImageHint,
    pub target_size: i32,
    pub target_PSNR: f32,
    pub segments: i32,
    pub sns_strength: i32,
    pub filter_strength: i32,
    pub filter_sharpness: i32,
    pub filter_type: i32,
    pub autofilter: i32,
    pub alpha_compression: i32,
    pub alpha_filtering: i32,
    pub alpha_quality: i32,
    pub pass: i32,
    pub show_compressed: i32,
    pub preprocessing: i32,
    pub partitions: i32,
    pub partition_limit: i32,
    pub emulate_jpeg_size: i32,
    pub thread_level: i32,
    pub low_memory: i32,
    pub near_lossless: i32,
    pub exact: i32,
    pub use_delta_palette: i32,
    pub use_sharp_yuv: i32,
    pub qmin: i32,
    pub qmax: i32,
}

impl OptionsTrait for WebPOptions {}

impl Default for WebPOptions {
    fn default() -> Self {
        Self {
            lossless: 0,
            quality: 75.0,
            method: 4,
            image_hint: WebPImageHint::WEBP_HINT_DEFAULT,
            target_size: 0,
            target_PSNR: 0.0,
            segments: 4,
            sns_strength: 50,
            filter_strength: 60,
            filter_sharpness: 0,
            filter_type: 1,
            autofilter: 0,
            alpha_compression: 1,
            alpha_filtering: 1,
            alpha_quality: 100,
            pass: 1,
            show_compressed: 0,
            preprocessing: 0,
            partitions: 0,
            partition_limit: 0,
            emulate_jpeg_size: 0,
            thread_level: 0,
            low_memory: 0,
            near_lossless: 100,
            exact: 0,
            use_delta_palette: 0,
            use_sharp_yuv: 0,
            qmin: 0,
            qmax: 100,
        }
    }
}

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
