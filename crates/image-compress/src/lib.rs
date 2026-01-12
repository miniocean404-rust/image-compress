//! # image_compress
//!
//! 图片压缩库，提供多种格式的图片压缩功能。
//!
//! ## 支持的格式
//!
//! - PNG (oxipng, imagequant)
//! - JPEG (mozjpeg) - 需要 `native` 特性
//! - WebP (libwebp) - 需要 `native` 特性
//! - AVIF (ravif) - 需要 `native` 特性

pub mod compress;
pub mod consts;

/// 重导出 consts 模块中的类型，提供更简洁的访问路径
pub mod support {
    pub use crate::consts::*;
}

/// 重导出 image_compress_core 中的编码器选项类型
pub mod export {
    // PNG 选项始终可用
    pub use image_compress_core::codecs::png::encoder::imagequant_options::*;
    pub use image_compress_core::codecs::png::encoder::oxipng_options::*;

    // native 特性下的选项导出
    #[cfg(feature = "native")]
    pub use image_compress_core::codecs::avif::encoder::options::*;
    #[cfg(feature = "native")]
    pub use image_compress_core::codecs::jpeg::encoder::options::*;
    #[cfg(feature = "native")]
    pub use image_compress_core::codecs::webp::encoder::options::*;
}

// 根级别也导出常用类型，保持向后兼容
pub use consts::{CompressState, Options, SupportedFileTypes};
pub use export::*;
