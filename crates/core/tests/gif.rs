//! GIF 编解码器测试模块
//!
//! 本模块测试 GIF 格式的编码和解码功能，包括：
//! - 解码器功能
//! - 无损编码（lossless）
//! - 有损编码（lossy）
//! - 不同色彩空间的编码支持（RGB、RGBA）
//! - 动画 GIF 编码
//! - 内存编码（encode_mem）
//! - GifOptions 配置选项测试

#![cfg(feature = "gif")]
#![allow(unused_imports)]
mod utils;

use utils::mock::*;
use utils::path::*;

use std::fs;
use std::io::BufReader;
use std::io::Cursor;

use image_compress_core::codecs::gif::decoder::GifDecoder;
use image_compress_core::codecs::gif::encoder::gifsicle::GifEncoder;
use image_compress_core::codecs::gif::encoder::options::GifOptions;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::traits::EncoderTrait;

/// 测试 GIF 解码器功能
///
/// 从文件读取 GIF 图像并解码，输出图像的尺寸、色彩空间和帧数信息。
/// 验证解码后的色彩空间是否为 RGBA。
#[test]
fn decode_gif() -> Result<(), Box<dyn std::error::Error>> {
    let byte_vec = fs::read(get_workspace_file_path("assets/image/gif/测试.gif"))?;
    let cursor = Cursor::new(&byte_vec);
    let reader = BufReader::new(cursor);
    let decoder = GifDecoder::try_new(reader)?;

    let img = Image::from_decoder(decoder)?;

    println!("GIF 尺寸: {:?}", img.dimensions());
    println!("GIF 色彩空间: {:?}", img.colorspace());
    println!("GIF 帧数: {}", img.frames_ref().len());

    assert_eq!(img.colorspace(), ColorSpace::RGBA);

    Ok(())
}

/// 测试 GIF 无损编码
///
/// 创建 100x100 的 RGBA 测试图像，使用无损压缩选项进行编码，
/// 验证编码过程是否成功完成。
#[test]
fn encode_gif_lossless() {
    let image = create_test_image_u8(100, 100, ColorSpace::RGBA);
    let mut encoder = GifEncoder::new_with_options(GifOptions::lossless());

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    if result.is_err() {
        dbg!(&result);
    }

    assert!(result.is_ok());
}

/// 测试 GIF 有损编码
///
/// 创建 100x100 的 RGBA 测试图像，使用质量为 80 的有损压缩选项进行编码，
/// 验证编码过程是否成功完成。
#[test]
fn encode_gif_lossy() {
    let image = create_test_image_u8(100, 100, ColorSpace::RGBA);
    let mut encoder = GifEncoder::new_with_options(GifOptions::lossy(80));

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    if result.is_err() {
        dbg!(&result);
    }

    assert!(result.is_ok());
}

/// 测试 RGB 色彩空间的 GIF 编码
///
/// 创建 100x100 的 RGB 测试图像（无 Alpha 通道），使用默认选项进行编码，
/// 验证编码器能够正确处理 RGB 色彩空间。
#[test]
fn encode_gif_rgb() {
    let image = create_test_image_u8(100, 100, ColorSpace::RGB);
    let mut encoder = GifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    if result.is_err() {
        dbg!(&result);
    }

    assert!(result.is_ok());
}

/// 测试动画 GIF 编码
///
/// 创建包含多帧的 100x100 RGB 动画图像，验证动画编码功能是否正常工作。
#[test]
fn encode_gif_animated() {
    let image = create_test_image_animated(100, 100, ColorSpace::RGB);
    let mut encoder = GifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    if result.is_err() {
        dbg!(&result);
    }

    assert!(result.is_ok());
}

/// 测试 GIF 内存编码功能
///
/// 从文件读取 GIF 图像，使用质量为 70 的有损压缩进行编码，
/// 并将结果写入输出文件。输出原始和压缩后的字节数以便比较压缩效果。
#[test]
fn encode_mem_gif() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_workspace_file_path("assets/image/gif/a.gif");
    let output_path = get_workspace_file_path("assets/compress/gif/a.gif");
    fs::create_dir_all(output_path.parent().unwrap())?;

    let read_buf = fs::read(input_path)?;

    let mut encoder = GifEncoder::new_with_options(GifOptions::lossy(70));

    let encode_buf = encoder.encode_mem(&read_buf)?;
    println!(
        "原始字节数: {} 压缩后字节数: {}",
        read_buf.len(),
        encode_buf.len()
    );

    fs::write(&output_path, &encode_buf)?;
    println!("输出路径: {:?}", output_path);

    Ok(())
}

/// 测试 GifOptions 默认配置
///
/// 验证 GifOptions::default() 返回的默认值是否正确：
/// - lossy: 0（无损）
/// - optimize_level: 2
/// - reduce_colors: false
/// - max_colors: 256
#[test]
fn gif_options_default() {
    let options = GifOptions::default();
    assert_eq!(options.lossy, 0);
    assert_eq!(options.optimize_level, 2);
    assert!(!options.reduce_colors);
    assert_eq!(options.max_colors, 256);
}

/// 测试 GifOptions 无损配置
///
/// 验证 GifOptions::lossless() 返回的配置是否正确：
/// - lossy: 0（无损）
/// - optimize_level: 3（最高优化级别）
#[test]
fn gif_options_lossless() {
    let options = GifOptions::lossless();
    assert_eq!(options.lossy, 0);
    assert_eq!(options.optimize_level, 3);
}

/// 测试 GifOptions 有损配置
///
/// 验证 GifOptions::lossy(50) 的 lossy 值计算是否正确。
/// 计算公式：(100 - quality) * 2，所以 lossy(50) 应该得到 100。
#[test]
fn gif_options_lossy() {
    let options = GifOptions::lossy(50);
    assert_eq!(options.lossy, 100); // (100 - 50) * 2 = 100
}

/// 测试 GifOptions 构建器模式
///
/// 验证 GifOptions 的链式调用构建器是否正确设置各个选项：
/// - with_lossy: 设置有损压缩级别
/// - with_optimize_level: 设置优化级别
/// - with_reduce_colors: 设置是否减少颜色数
/// - with_max_colors: 设置最大颜色数
#[test]
fn gif_options_builder() {
    let options = GifOptions::default()
        .with_lossy(50)
        .with_optimize_level(3)
        .with_reduce_colors(true)
        .with_max_colors(128);

    assert_eq!(options.lossy, 50);
    assert_eq!(options.optimize_level, 3);
    assert!(options.reduce_colors);
    assert_eq!(options.max_colors, 128);
}
