// jSquash 写法
use std::io::Cursor;

use rgb::{
    alt::{Gray, GrayAlpha},
    AsPixels, FromSlice, RGB8, RGBA8,
};

use crate::error::{CompressError, Result};

pub fn encode(data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    let mut buf = Vec::new();

    {
        let mut encoder = png::Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder
            .write_header()
            .map_err(|e| CompressError::PngEncode(format!("写入 PNG 头失败: {}", e)))?;
        writer
            .write_image_data(data)
            .map_err(|e| CompressError::PngEncode(format!("写入 PNG 数据失败: {}", e)))?;
    }

    Ok(buf)
}

pub fn decode(data: &[u8]) -> Result<Vec<u8>> {
    let cursor = Cursor::new(data);
    let mut decoder = png::Decoder::new(cursor);
    decoder.set_transformations(
        png::Transformations::EXPAND | // Turn images <8bit to 8bit
        png::Transformations::STRIP_16, // Turn 16bit into 8 bit
    );

    let mut reader = decoder
        .read_info()
        .map_err(|e| CompressError::PngDecode(format!("读取 PNG 信息失败: {}", e)))?;
    let output_size = reader
        .output_buffer_size()
        .ok_or_else(|| CompressError::PngDecode("无法获取输出缓冲区大小".to_string()))?;
    let mut buf = vec![0; output_size];

    reader
        .next_frame(&mut buf)
        .map_err(|e| CompressError::PngDecode(format!("读取 PNG 帧失败: {}", e)))?;

    let info = reader.info();

    // Transformations::EXPAND will expand indexed palettes and lower-bit
    // grayscales to higher color types, but we still need to transform
    // the rest to RGBA.
    match info.color_type {
        png::ColorType::Rgba => {}
        png::ColorType::Rgb => expand_pixels(&mut buf, RGB8::into),
        png::ColorType::GrayscaleAlpha => expand_pixels(&mut buf, Gray::<u8>::into),
        png::ColorType::Grayscale => expand_pixels(&mut buf, |gray: GrayAlpha<u8>| gray.into()),
        png::ColorType::Indexed => {
            return Err(CompressError::PngDecode(
                "找到已索引的颜色类型，但期望它已经展开".to_string(),
            ));
        }
    }

    Ok(buf)
}

// Convert pixels in-place within buffer containing source data but preallocated
// for entire [num_pixels * sizeof(RGBA)].
// This works because all the color types are <= RGBA by size.
fn expand_pixels<Src: Copy>(buf: &mut [u8], to_rgba: impl Fn(Src) -> RGBA8)
where
    [u8]: AsPixels<Src> + FromSlice<u8>,
{
    assert!(std::mem::size_of::<Src>() <= std::mem::size_of::<RGBA8>());
    let num_pixels = buf.len() / 4;
    for i in (0..num_pixels).rev() {
        let src_pixel = buf.as_pixels()[i];
        buf.as_rgba_mut()[i] = to_rgba(src_pixel);
    }
}
