use anyhow::anyhow;
use image_compress_core::codecs::{
    avif::{self, encoder::AvifEncoder},
    jpeg::mozjpeg::{self, MozJpegEncoder},
    png::{
        imagequant::{self, ImageQuantEncoder},
        oxipng::{self, OxiPngEncoder},
    },
    webp::{self, encoder::WebPEncoder},
};
use utils::file::mime::get_mime_for_memory;

use crate::{state::CompressState, support::SupportedFileTypes};

pub type OxiPngOptions = oxipng::OxiPngOptions;
pub type ImageQuantOptions = imagequant::ImageQuantOptions;

pub type MozJpegOptions = mozjpeg::MozJpegOptions;

pub type WebPOptions = webp::encoder::WebPOptions;

pub type AvifOptions = avif::encoder::AvifOptions;

#[derive(Debug, Default)]
pub struct ImageCompress {
    pub image: Vec<u8>,

    pub compress_image: Vec<u8>,

    pub ext: SupportedFileTypes,

    pub state: CompressState,

    pub quality: u8,

    pub before_size: usize,

    pub after_size: usize,

    pub rate: f64,
    // oxipng_options: OxiPngOptions,
    // image_quant_options: ImageQuantOptions,
    // mozjpeg_options: MozJpegOptions,
    // webp_options: WebPOptions,
    // avif_options: AvifOptions,
}

impl ImageCompress {
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

    pub fn with_oxipng_options(&mut self, options: OxiPngOptions) -> anyhow::Result<Vec<u8>> {
        self.pre_compress();
        self.compress_image = OxiPngEncoder::new_with_options(options).encode_mem(&self.image)?;
        self.post_compress();

        Ok(self.compress_image.clone())
    }

    pub fn with_image_quant_options(
        &mut self,
        options: ImageQuantOptions,
    ) -> anyhow::Result<Vec<u8>> {
        self.pre_compress();
        self.compress_image =
            ImageQuantEncoder::new_with_options(options).encode_mem(&self.image)?;
        self.post_compress();

        Ok(self.compress_image.clone())
    }

    pub fn with_mozjpeg_options(&mut self, options: MozJpegOptions) -> anyhow::Result<Vec<u8>> {
        self.pre_compress();
        self.compress_image = MozJpegEncoder::new_with_options(options).encode_mem(&self.image)?;
        self.post_compress();

        Ok(self.compress_image.clone())
    }

    pub fn with_webp_options(&mut self, options: WebPOptions) -> anyhow::Result<Vec<u8>> {
        self.pre_compress();
        self.compress_image = WebPEncoder::new_with_options(options).encode_mem(&self.image)?;
        self.post_compress();

        Ok(self.compress_image.clone())
    }

    pub fn with_avif_options(&mut self, options: AvifOptions) -> anyhow::Result<Vec<u8>> {
        self.pre_compress();
        self.compress_image = AvifEncoder::new_with_options(options).encode_mem(&self.image)?;
        self.post_compress();

        Ok(self.compress_image.clone())
    }

    fn pre_compress(&mut self) {
        self.state = CompressState::Compressing;
    }

    fn post_compress(&mut self) {
        self.after_size = self.compress_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;
    }

    // pub fn compress_with_mem(&mut self) -> anyhow::Result<()> {
    //     self.state = CompressState::Compressing;

    //     self.compress_image = match self.ext {
    //         SupportedFileTypes::Jpeg => MozJpegEncoder::new().encode_mem(&self.image),
    //         SupportedFileTypes::Png => OxiPngEncoder::new().encode_mem(&self.image),
    //         SupportedFileTypes::WebP => WebPEncoder::new().encode_mem(&self.image),
    //         SupportedFileTypes::Avif => AvifEncoder::new().encode_mem(&self.image),
    //         SupportedFileTypes::Unknown => Err(anyhow!("不能压缩的类型")),
    //     }?;

    //     self.after_size = self.compress_image.len();

    //     self.rate = (((self.before_size as f64 - self.after_size as f64)
    //         / self.before_size as f64)
    //         * 10000.0)
    //         .round()
    //         / 100.0;

    //     self.state = CompressState::Done;

    //     Ok(())
    // }
}
