use std::any::Any;

use anyhow::anyhow;
use image_compress_core::codecs::{
    avif::{self, encoder::AvifEncoder},
    jpeg::{self, mozjpeg::MozJpegEncoder},
    png::{
        imagequant::{self, ImageQuantEncoder},
        oxipng::{self, OxiPngEncoder},
    },
    webp::{self},
    OptionsTrait,
};
use utils::file::mime::get_mime_for_memory;

use crate::{state::CompressState, support::SupportedFileTypes};

pub type OxiPngOptions = oxipng::OxiPngOptions;
pub type ImageQuantOptions = imagequant::ImageQuantOptions;

pub type MozJpegOptions = jpeg::mozjpeg::MozJpegOptions;

pub type WebPOptions = webp::encoder::WebPOptions;

pub type AvifOptions = avif::encoder::AvifOptions;

#[derive(Debug, Default)]
pub struct ImageCompress<O>
where
    O: OptionsTrait,
{
    pub image: Vec<u8>,

    pub compress_image: Vec<u8>,

    pub ext: SupportedFileTypes,

    pub state: CompressState,

    pub quality: u8,

    pub before_size: usize,

    pub after_size: usize,

    pub rate: f64,

    options: O,
}

impl<O: OptionsTrait> ImageCompress<O> {
    pub fn new(buffer: Vec<u8>, quality: u8) -> Self {
        let before_size = buffer.len();
        let ext = get_mime_for_memory(&buffer).into();

        Self {
            image: buffer,
            quality,
            before_size,
            ext,
            ..Default::default()
        }
    }

    pub fn with_options(self, options: O) -> Self {
        Self { options, ..self }
    }

    pub fn compress(mut self) -> anyhow::Result<Vec<u8>> {
        self.state = CompressState::Compressing;

        let options = Box::new(self.options) as Box<dyn Any>;

        self.compress_image = match self.ext {
            SupportedFileTypes::Jpeg => {
                let options = *options
                    .downcast::<MozJpegOptions>()
                    .map_err(|_| anyhow!("Any downcast 转换错误"))?;
                MozJpegEncoder::new_with_options(options).encode_mem(&self.image)
            }
            SupportedFileTypes::Png => {
                if options.is::<OxiPngOptions>() {
                    let options = *options
                        .downcast::<OxiPngOptions>()
                        .map_err(|_| anyhow!("Any downcast 转换错误"))?;
                    OxiPngEncoder::new_with_options(options).encode_mem(&self.image)
                } else {
                    let options = *options
                        .downcast::<ImageQuantOptions>()
                        .map_err(|_| anyhow!("Any downcast 转换错误"))?;
                    ImageQuantEncoder::new_with_options(options).encode_mem(&self.image)
                }
            }
            SupportedFileTypes::WebP => {
                let options = *options
                    .downcast::<WebPOptions>()
                    .map_err(|_| anyhow!("Any downcast 转换错误"))?;
                webp::encoder::WebPEncoder::new_with_options(options).encode_mem(&self.image)
            }
            SupportedFileTypes::Avif => {
                let options = *options
                    .downcast::<AvifOptions>()
                    .map_err(|_| anyhow!("Any downcast 转换错误"))?;
                AvifEncoder::new_with_options(options).encode_mem(&self.image)
            }
            SupportedFileTypes::Unknown => Err(anyhow!("不能压缩的类型")),
        }?;

        self.after_size = self.compress_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;

        Ok(self.compress_image.to_vec())
    }
}
