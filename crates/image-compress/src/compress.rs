use std::fmt::{self};

use image_compress_core::codecs::png::encoder::{
    imagequant::ImageQuantEncoder, oxipng::OxiPngEncoder,
};

#[cfg(feature = "native")]
use image_compress_core::codecs::{
    avif::encoder::ravif::AvifEncoder, gif::encoder::gifsicle::GifEncoder,
    jpeg::encoder::mozjpeg::MozJpegEncoder, webp::encoder::webp::WebPEncoder,
};
use image_compress_core::error::CompressError;
use utils::file::mime::get_mime_for_memory;

use crate::consts::{CompressState, Options, SupportedFileTypes};

/// 图片压缩器
///
/// 提供图片压缩的核心功能，支持多种图片格式。
pub struct ImageCompress {
    /// 原始图片数据
    pub image: Vec<u8>,

    /// 压缩后的图片数据
    pub compressed_image: Vec<u8>,

    /// 图片格式类型
    pub ext: SupportedFileTypes,

    /// 当前压缩状态
    pub state: CompressState,

    /// 压缩质量 (0-100)
    pub quality: u8,

    /// 压缩前文件大小 (字节)
    pub before_size: usize,

    /// 压缩后文件大小 (字节)
    pub after_size: usize,

    /// 压缩率 (百分比，正值表示体积减小)
    pub rate: f64,

    /// 压缩选项配置
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

impl ImageCompress {
    /// 创建一个新的图片压缩器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置要压缩的图片数据
    ///
    /// ### 参数
    ///
    /// * `image` - 原始图片的字节数据
    ///
    /// ### 返回
    ///
    /// 返回配置了图片数据的 `ImageCompress` 实例
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

    /// 设置压缩选项
    ///
    /// ### 参数
    ///
    /// * `options` - 压缩选项，根据图片格式选择对应的选项类型
    ///
    /// ### 返回
    ///
    /// 返回配置了压缩选项的 `ImageCompress` 实例
    pub fn with_options(self, options: Options) -> Self {
        Self { options, ..self }
    }

    /// 执行图片压缩
    ///
    /// 根据设置的选项对图片进行压缩，并更新压缩状态和统计信息。
    ///
    /// ### 返回
    ///
    /// * `Ok(Vec<u8>)` - 压缩成功，返回压缩后的图片数据
    /// * `Err` - 压缩失败，返回错误信息
    ///
    /// ### 错误
    ///
    /// * 未设置压缩选项时返回 `InvalidParameter` 错误
    /// * 编码器内部错误
    pub fn compress(&mut self) -> anyhow::Result<Vec<u8>> {
        self.state = CompressState::Compressing;

        self.compressed_image = match &self.options {
            Options::OxiPng(options) => {
                OxiPngEncoder::new_with_options(options.clone()).encode_mem(&self.image)
            }
            Options::ImageQuant(options) => {
                ImageQuantEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            #[cfg(feature = "native")]
            Options::MozJpeg(options) => {
                MozJpegEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            #[cfg(feature = "native")]
            Options::WebP(options) => {
                WebPEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            #[cfg(feature = "native")]
            Options::Avif(options) => {
                AvifEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            #[cfg(feature = "native")]
            Options::Gif(options) => {
                GifEncoder::new_with_options(*options).encode_mem(&self.image)
            }
            Options::Unknown => Err(CompressError::InvalidParameter(
                "没有设置 options 或 不能压缩的类型".to_string(),
            )),
        }?;

        self.after_size = self.compressed_image.len();

        self.rate = (((self.before_size as f64 - self.after_size as f64)
            / self.before_size as f64)
            * 10000.0)
            .round()
            / 100.0;

        self.state = CompressState::Done;

        Ok(self.compressed_image.clone())
    }
}
