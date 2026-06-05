//! AVIF 编解码器测试模块
//!
//! 本模块测试 AVIF 格式的编码和解码功能，包括：
//! - 内存编码（encode_mem）
//! - 解码器功能
//! - 不同色彩空间的编码支持（RGB、RGBA 等）
//! - 不同位深度的编码支持（u8、u16、f32）
//! - 动画图像编码

#![cfg(feature = "avif")]

use std::{
    fs::{self},
    io::Cursor,
};
mod utils;

use image_compress_core::codecs::avif::{decoder::AvifDecoder, encoder::ravif::AvifEncoder};
use utils::{mock::*, path::get_workspace_file_path};
use zune_core::colorspace::ColorSpace;
use zune_image::{image::Image, traits::EncoderTrait};

/// 测试 AVIF 内存编码功能
///
/// 从文件读取 AVIF 图像，使用默认参数进行压缩编码，
/// 并将结果写入输出文件。验证编码过程是否成功完成。
#[test]
fn encode_mem_avif() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_workspace_file_path("assets/image/avif/测试.avif");
    let output_path = get_workspace_file_path("assets/compress/avif/测试.avif");
    fs::create_dir_all(output_path.parent().expect("output path should have parent directory"))?;

    let read_buf = fs::read(input_path)?;

    let mut encoder = AvifEncoder::new();

    let encode_buf = encoder.encode_mem(&read_buf)?;
    // 默认参数结果: 原始字节数: 317 压缩后字节数: 306
    // println!("原始字节数: {} 压缩后字节数: {}", read_buf.len(), encode_buf.len());

    fs::write(&output_path, &encode_buf)?;
    // println!("输出路径: {:?}", output_path);

    Ok(())
}

/// 测试 AVIF 解码器功能
///
/// 从文件读取 AVIF 图像并解码，验证解码后的图像尺寸和色彩空间是否正确。
/// 预期结果：尺寸为 48x80，色彩空间为 RGBA。
#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let buf = fs::read(get_workspace_file_path("assets/image/avif/测试.avif"))?;
    let cursor = Cursor::new(&buf);
    let decoder = AvifDecoder::try_new(cursor)?;

    let image = Image::from_decoder(decoder)?;

    assert_eq!(image.dimensions(), (48, 80));
    assert_eq!(image.colorspace(), ColorSpace::RGBA);
    Ok(())
}

/// 测试 u8 位深度下所有支持的色彩空间编码
///
/// 遍历编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u8 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u8() {
    let mut results = vec![];

    let encoder = AvifEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u8(200, 200, *colorspace);

                let mut encoder = AvifEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                // if result.is_err() {
                //     dbg!(&result);
                // }

                assert!(result.is_ok());
            })
            .expect("spawn colorspace encoder test thread");

        results.push(handler.join())
    }

    results
        .into_iter()
        .collect::<Result<Vec<()>, _>>()
        .expect("colorspace encoder test thread should not panic");
}

/// 测试 u16 位深度下所有支持的色彩空间编码
///
/// 遍历编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u16 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u16() {
    let mut results = vec![];

    let encoder = AvifEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = AvifEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                // if result.is_err() {
                //     dbg!(&result);
                // }

                assert!(result.is_ok());
            })
            .expect("spawn colorspace encoder test thread");

        results.push(handler.join())
    }

    results
        .into_iter()
        .collect::<Result<Vec<()>, _>>()
        .expect("colorspace encoder test thread should not panic");
}

/// 测试 f32 位深度下所有支持的色彩空间编码
///
/// 遍历编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 f32 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_f32() {
    let mut results = vec![];

    let encoder = AvifEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = AvifEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                // if result.is_err() {
                //     dbg!(&result);
                // }

                assert!(result.is_ok());
            })
            .expect("spawn colorspace encoder test thread");

        results.push(handler.join())
    }

    results
        .into_iter()
        .collect::<Result<Vec<()>, _>>()
        .expect("colorspace encoder test thread should not panic");
}

/// 测试 u8 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB u8 测试图像，验证基本编码功能是否正常工作。
#[test]
fn encode_u8() {
    let image = create_test_image_u8(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    // dbg!(&result);

    assert!(result.is_ok());
}

/// 测试 u16 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB u16 测试图像，验证 16 位深度编码功能是否正常工作。
#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    // dbg!(&result);

    assert!(result.is_ok());
}

/// 测试 f32 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB f32 测试图像，验证浮点位深度编码功能是否正常工作。
#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    // dbg!(&result);

    assert!(result.is_ok());
}

/// 测试动画 AVIF 编码
///
/// 创建包含多帧的 200x200 RGB 动画图像，验证动画编码功能是否正常工作。
#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    // dbg!(&result);

    assert!(result.is_ok());
}
