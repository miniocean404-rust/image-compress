//! JPEG 编解码器测试模块
//!
//! 本模块测试 JPEG 格式的编码功能，使用 MozJpeg 编码器，包括：
//! - 内存编码（encode_mem）
//! - 不同色彩空间的编码支持
//! - 不同位深度的编码支持（u8、u16、f32）
//! - 动画图像编码（JPEG 不支持动画，但测试编码器的容错能力）

#![cfg(feature = "jpeg")]
#![allow(unused_imports)]
mod utils;

use utils::mock::*;
use utils::path::*;

use image_compress_core::codecs::jpeg::encoder::mozjpeg::MozJpegEncoder;
use std::rc::Rc;
use std::{fs, io::Cursor};
use zune_core::result;
use zune_core::{colorspace::ColorSpace, options::DecoderOptions};
use zune_image::errors::ImageErrors;
use zune_image::image::Image;
use zune_image::traits::EncoderTrait;

/// 测试 JPEG 内存编码功能
///
/// 从文件读取 JPEG 图像，使用 MozJpeg 编码器进行压缩，
/// 并将结果写入输出文件。输出原始和压缩后的字节数以便比较压缩效果。
#[test]
fn encode_mem_jpeg() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_workspace_file_path("assets/image/jpeg/测试.jpg");
    let output_path = get_workspace_file_path("assets/compress/jpeg/测试-已压缩.jpg");
    fs::create_dir_all(output_path.parent().unwrap())?;

    let read_buf = fs::read(input_path)?;

    let mut encoder = MozJpegEncoder::new();

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

/// 测试 u8 位深度下所有支持的色彩空间编码
///
/// 遍历 MozJpeg 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u8 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u8() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u8(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

/// 测试 u16 位深度下所有支持的色彩空间编码
///
/// 遍历 MozJpeg 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u16 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u16() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

/// 测试 f32 位深度下所有支持的色彩空间编码
///
/// 遍历 MozJpeg 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 f32 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_f32() {
    let mut results = vec![];

    let encoder = MozJpegEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = MozJpegEncoder::new();

                let buf = Cursor::new(vec![]);

                let result = encoder.encode(&image, buf);

                if result.is_err() {
                    dbg!(&result);
                }

                assert!(result.is_ok());
            })
            .unwrap();

        results.push(handler.join())
    }

    results.into_iter().collect::<Result<Vec<()>, _>>().unwrap();
}

/// 测试 u8 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB u8 测试图像，验证基本编码功能是否正常工作。
#[test]
fn encode_u8() {
    let image = create_test_image_u8(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

/// 测试 u16 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB u16 测试图像，验证 16 位深度编码功能是否正常工作。
#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

/// 测试 f32 位深度 RGB 色彩空间的基本编码
///
/// 创建 200x200 的 RGB f32 测试图像，验证浮点位深度编码功能是否正常工作。
#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

/// 测试动画图像编码
///
/// 创建包含多帧的 200x200 RGB 动画图像，测试编码器对多帧图像的处理能力。
/// 注意：JPEG 格式本身不支持动画，此测试验证编码器的容错处理。
#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
