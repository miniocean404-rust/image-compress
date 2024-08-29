use anyhow::anyhow;
use image_compress_core::codecs::{
    avif::encoder::AvifEncoder,
    jpeg::mozjpeg::MozJpegEncoder,
    png::oxipng::{OxiPngEncoder, OxiPngOptions},
    webp::encoder::WebPEncoder,
};
use utils::file::mime::get_mime_for_memory;

use crate::{state::CompressState, support::SupportedFileTypes};

#[derive(Clone, Debug, Default)]
pub struct ImageCompress {
    pub image: Vec<u8>,

    pub compress_image: Vec<u8>,

    pub ext: SupportedFileTypes,

    pub state: CompressState,

    pub quality: u8,

    pub before_size: usize,

    pub after_size: usize,

    pub rate: f64,

    pub oxipng_options: OxiPngOptions,
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

    pub fn with_oxipng_options(&mut self, options: OxiPngOptions) {
        self.oxipng_options = options;
    }

    pub fn compress_with_mem(&mut self) -> anyhow::Result<()> {
        self.state = CompressState::Compressing;

        self.compress_image = match self.ext {
            SupportedFileTypes::Jpeg => MozJpegEncoder::new().encode_mem(&self.image),
            SupportedFileTypes::Png => OxiPngEncoder::new().encode_mem(&self.image),
            SupportedFileTypes::WebP => WebPEncoder::new().encode_mem(&self.image),
            SupportedFileTypes::Avif => AvifEncoder::new().encode_mem(&self.image),
            SupportedFileTypes::Unknown => Err(anyhow!("不能压缩的类型")),
        }?;

        self.after_size = self.compress_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;

        Ok(())
    }
}
