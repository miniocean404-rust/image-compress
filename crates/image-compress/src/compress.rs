use std::fmt::{self};

use anyhow::anyhow;
use image_compress_core::codecs::{
    avif::encoder::ravif::AvifEncoder,
    jpeg::encoder::mozjpeg::MozJpegEncoder,
    png::encoder::{imagequant::ImageQuantEncoder, oxipng::OxiPngEncoder},
    webp::encoder::webp::WebPEncoder,
};
use utils::file::mime::get_mime_for_memory;

use crate::export::*;
use crate::{state::CompressState, support::SupportedFileTypes};

#[derive(Debug, Clone)]
pub enum Options {
    MozJpeg(MozJpegOptions),
    OxiPng(OxiPngOptions),
    ImageQuant(ImageQuantOptions),
    WebP(WebPOptions),
    Avif(AvifOptions),
    Unknown,
}

pub struct ImageCompress {
    pub image: Vec<u8>,

    pub compressed_image: Vec<u8>,

    pub ext: SupportedFileTypes,

    pub state: CompressState,

    pub quality: u8,

    pub before_size: usize,

    pub after_size: usize,

    pub rate: f64,

    options: Options,
}

impl Default for ImageCompress {
    fn default() -> Self {
        Self {
            image: vec![],
            before_size: 0,
            ext: SupportedFileTypes::Unknown,
            options: Options::Unknown,
            compressed_image: vec![],
            state: CompressState::Ready,
            quality: 0,
            after_size: 0,
            rate: 0.0,
        }
    }
}

impl ImageCompress {
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

    pub fn with_options(self, options: Options) -> Self {
        Self { options, ..self }
    }

    pub fn compress(&mut self) -> anyhow::Result<Vec<u8>> {
        self.state = CompressState::Compressing;

        self.compressed_image = match self.options.clone() {
            Options::MozJpeg(options) => {
                MozJpegEncoder::new_with_options(options).encode_mem(&self.image)
            }
            Options::OxiPng(options) => {
                OxiPngEncoder::new_with_options((options).clone()).encode_mem(&self.image)
            }
            Options::ImageQuant(options) => {
                ImageQuantEncoder::new_with_options(options).encode_mem(&self.image)
            }
            Options::WebP(options) => {
                WebPEncoder::new_with_options(options).encode_mem(&self.image)
            }
            Options::Avif(options) => {
                AvifEncoder::new_with_options(options).encode_mem(&self.image)
            }
            Options::Unknown => Err(anyhow!("没有设置 options 或 不能压缩的类型")),
        }?;

        self.after_size = self.compressed_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;

        Ok(self.compressed_image.clone().to_vec())
    }
}

impl fmt::Display for ImageCompress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl fmt::Debug for ImageCompress {
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
