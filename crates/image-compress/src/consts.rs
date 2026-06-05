use image_compress_core::codecs::png::encoder::{imagequant_options::ImageQuantOptions, oxipng_options::OxiPngOptions};
#[cfg(feature = "native")]
use image_compress_core::codecs::{
    avif::encoder::options::AvifOptions, gif::encoder::options::GifOptions, jpeg::encoder::options::MozJpegOptions,
    webp::encoder::options::WebPOptions,
};

#[derive(Clone, Debug, Default)]
pub enum CompressState {
    #[default]
    Ready,
    Compressing,
    Done,
}

/// 压缩选项枚举
///
/// 根据不同的图片格式选择对应的压缩选项
#[derive(Debug, Clone)]
pub enum Options {
    /// PNG 格式压缩选项 (使用 oxipng)
    OxiPng(OxiPngOptions),
    /// PNG 格式量化压缩选项 (使用 imagequant)
    ImageQuant(ImageQuantOptions),
    /// JPEG 格式压缩选项 (使用 mozjpeg)
    #[cfg(feature = "native")]
    MozJpeg(MozJpegOptions),
    /// WebP 格式压缩选项 (使用 libwebp)
    #[cfg(feature = "native")]
    WebP(WebPOptions),
    /// AVIF 格式压缩选项 (使用 ravif)
    #[cfg(feature = "native")]
    Avif(AvifOptions),
    /// GIF 格式压缩选项 (使用 gifsicle)
    #[cfg(feature = "native")]
    Gif(GifOptions),
    /// 未知/未设置选项
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub enum SupportedFileTypes {
    Jpeg,
    Png,
    WebP,
    Avif,
    Gif,

    // 为 Default 宏设置默认值
    #[default]
    Unknown,
}

impl From<&str> for SupportedFileTypes {
    fn from(mime: &str) -> Self {
        match mime {
            "image/jpeg" => SupportedFileTypes::Jpeg,
            "image/png" => SupportedFileTypes::Png,
            "image/webp" => SupportedFileTypes::WebP,
            "image/avif" => SupportedFileTypes::Avif,
            "image/gif" => SupportedFileTypes::Gif,
            _ => SupportedFileTypes::Unknown,
        }
    }
}
