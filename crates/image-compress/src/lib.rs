use image_compress_core::codecs::{avif, jpeg, png, webp};

pub use avif::encoder::options::*;
pub use jpeg::encoder::options::*;
pub use png::encoder::imagequant_options::*;
pub use png::encoder::oxipng_options::*;
pub use webp::encoder::options::*;

pub mod compress;
pub mod consts;
