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

use image_compress_core::codecs::png;

#[cfg(feature = "native")]
use image_compress_core::codecs::{avif, jpeg, webp};

// PNG 选项始终可用
pub use png::encoder::imagequant_options::*;
pub use png::encoder::oxipng_options::*;

// native 特性下的选项导出
#[cfg(feature = "native")]
pub use avif::encoder::options::*;
#[cfg(feature = "native")]
pub use jpeg::encoder::options::*;
#[cfg(feature = "native")]
pub use webp::encoder::options::*;

pub mod compress;
pub mod consts;
