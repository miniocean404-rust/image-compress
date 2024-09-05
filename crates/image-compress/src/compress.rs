pub use image_compress_core::codecs::OptionsTrait;

use std::{
    any::Any,
    fmt::{self},
    sync::Arc,
};

use anyhow::anyhow;
use image_compress_core::codecs::{
    avif::encoder::ravif::AvifEncoder,
    jpeg::encoder::mozjpeg::MozJpegEncoder,
    png::encoder::{imagequant::ImageQuantEncoder, oxipng::OxiPngEncoder},
    webp::{self},
};
use utils::file::mime::get_mime_for_memory;

use crate::export::*;
use crate::{state::CompressState, support::SupportedFileTypes};

pub enum Options {
    MozJpeg(MozJpegOptions),
    OxiPng(OxiPngOptions),
    ImageQuant(ImageQuantOptions),
    WebP(WebPOptions),
    Avif(AvifOptions),
    Unknown,
}

pub struct ImageCompress<O>
where
    O: OptionsTrait,
{
    pub image: Vec<u8>,

    pub compressed_image: Vec<u8>,

    pub ext: SupportedFileTypes,

    pub state: CompressState,

    pub quality: u8,

    pub before_size: usize,

    pub after_size: usize,

    pub rate: f64,

    options: Option<Arc<O>>,
}

impl<O: OptionsTrait> Default for ImageCompress<O> {
    fn default() -> Self {
        Self {
            image: vec![],
            before_size: 0,
            ext: SupportedFileTypes::Unknown,
            options: None,
            compressed_image: vec![],
            state: CompressState::Ready,
            quality: 0,
            after_size: 0,
            rate: 0.0,
        }
    }
}

impl<O: OptionsTrait> ImageCompress<O> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_buffer(self, image: Vec<u8>) -> Self {
        let before_size = image.len();
        let ext = get_mime_for_memory(&image).into();

        Self {
            image,
            before_size,
            ext,
            ..self
        }
    }

    pub fn with_options(self, options: O) -> Self {
        Self {
            options: Some(Arc::new(options)),
            ..self
        }
    }

    pub fn compress(&mut self) -> anyhow::Result<&Vec<u8>> {
        self.state = CompressState::Compressing;
        let option_arc = self
            .options
            .clone()
            .ok_or(anyhow!("没有赋值 options"))?
            .clone();

        let options = option_arc as Arc<dyn Any>;

        self.compressed_image = match self.ext {
            SupportedFileTypes::Jpeg => {
                let options = options
                    .downcast_ref::<MozJpegOptions>()
                    .ok_or(anyhow!("Any downcast 转换错误"))?;

                MozJpegEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            SupportedFileTypes::Png => {
                if options.is::<Arc<OxiPngOptions>>() {
                    let options = options
                        .downcast_ref::<OxiPngOptions>()
                        .ok_or(anyhow!("Any downcast 转换错误"))?;

                    OxiPngEncoder::new_with_options((*options).clone()).encode_mem(&self.image)
                } else {
                    let options = options
                        .downcast_ref::<ImageQuantOptions>()
                        .ok_or(anyhow!("Any downcast 转换错误"))?;

                    ImageQuantEncoder::new_with_options(*options).encode_mem(&self.image)
                }
            }
            SupportedFileTypes::WebP => {
                let options = options
                    .downcast_ref::<WebPOptions>()
                    .ok_or(anyhow!("Any downcast 转换错误"))?;
                webp::encoder::webp::WebPEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            SupportedFileTypes::Avif => {
                let options = options
                    .downcast_ref::<AvifOptions>()
                    .ok_or(anyhow!("Any downcast 转换错误"))?;
                AvifEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            SupportedFileTypes::Unknown => Err(anyhow!("不能压缩的类型")),
        }?;

        self.after_size = self.compressed_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;

        Ok(&self.compressed_image)
    }
}

impl<O> fmt::Display for ImageCompress<O>
where
    O: OptionsTrait,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<O> fmt::Debug for ImageCompress<O>
where
    O: OptionsTrait,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCompress")
            .field("ext", &self.ext)
            .field("state", &self.state)
            .field("quality", &self.quality)
            .field("before_size", &self.before_size)
            .field("after_size", &self.after_size)
            .field("rate", &self.rate)
            .field("options", &self.options)
            .finish()
    }
}
