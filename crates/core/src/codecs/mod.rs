#[cfg(feature = "avif")]
pub mod avif;
#[cfg(feature = "gif")]
pub mod gif;
#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "png")]
pub mod png;
#[cfg(feature = "webp")]
pub mod webp;
// 无压缩编码，只有解码
#[cfg(feature = "tiff")]
pub mod tiff;