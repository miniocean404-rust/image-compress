use std::io::Cursor;

use zune_core::options::{DecoderOptions, EncoderOptions};
// Jxl encoder 尺寸大，但是是无损格式
pub use zune_image::codecs::jpeg_xl::JxlEncoder;
use zune_image::{errors::ImageErrors, image::Image, traits::EncoderTrait};

// 编码为 Jxl 格式
pub fn encode_mem_jxl(buf: &Vec<u8>, options: EncoderOptions) -> Result<Vec<u8>, ImageErrors> {
    let cursor = Cursor::new(buf);

    let image = Image::read(cursor, DecoderOptions::default())?;

    let mut compress_buf = Cursor::new(vec![]);
    let mut encoder = JxlEncoder::new_with_options(options);
    encoder.encode(&image, &mut compress_buf)?;

    Ok(compress_buf.into_inner())
}
