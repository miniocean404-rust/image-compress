//! TIFF 编解码器测试模块
//!
//! 本模块测试 TIFF 格式的解码功能。
//! 注意：当前仅支持 TIFF 解码，不支持编码。

#![cfg(feature = "tiff")]
#![allow(unused_imports)]
mod utils;
use utils::mock::*;
use utils::path::*;

use image_compress_core::codecs::tiff::decoder::TiffDecoder;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::traits::EncoderTrait;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;

/// 测试 TIFF 解码器功能
///
/// 从文件读取 TIFF 图像并解码，验证解码后的图像尺寸和色彩空间是否正确。
/// 预期结果：尺寸为 48x80，色彩空间为 RGB。
#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let byte_vec = fs::read(get_workspace_file_path("assets/image/tiff/f1t.tif"))?;
    let cursor = Cursor::new(&byte_vec);
    let reader = BufReader::new(cursor);

    let decoder = TiffDecoder::try_new(reader)?;

    let image = Image::from_decoder(decoder)?;

    assert_eq!(image.dimensions(), (48, 80));
    assert_eq!(image.colorspace(), ColorSpace::RGB);

    Ok(())
}
