//! PNG 编解码器测试模块
//!
//! 本模块测试 PNG 格式的编码功能，包括：
//! - ImageQuant 有损压缩（颜色量化）
//! - OxiPng 无损压缩
//! - 双重压缩（先无损后有损）
//! - 不同色彩空间的编码支持
//! - 不同位深度的编码支持（u8、u16、f32）
//! - 动画 PNG 编码

#![allow(unused_imports)]
mod utils;

use utils::mock::*;
use utils::path::*;

use image_compress_core::codecs::png::encoder::imagequant::ImageQuantEncoder;
use image_compress_core::codecs::png::encoder::oxipng::OxiPngEncoder;
use std::{fs, io::Cursor};
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::ImageFormat;
use zune_image::traits::EncoderTrait;

use image_compress_core::codecs::png::encoder::imagequant_options::ImageQuantOptions;

/// 测试 ImageQuant 有损压缩
///
/// 使用 ImageQuant 编码器对 PNG 图像进行有损压缩（颜色量化），
/// 通过减少颜色数量来实现更高的压缩率。输出原始和压缩后的字节数。
#[test]
fn image_quant_compress_lossy() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_workspace_file_path("assets/image/png/测试.png");
    let output_path = get_workspace_file_path("assets/compress/png/测试-已压缩.png");
    fs::create_dir_all(output_path.parent().unwrap())?;

    let read_buf = fs::read(input_path)?;

    let mut encoder = ImageQuantEncoder::new();

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

/// 测试 OxiPng 无损压缩
///
/// 使用 OxiPng 编码器对 PNG 图像进行无损压缩，采用最大压缩级别。
/// 无损压缩不会损失任何图像质量，但压缩率相对有损压缩较低。
#[test]
fn oxipng_compress_lossless() {
    let buf = fs::read(get_workspace_file_path("assets/image/png/测试.png")).unwrap();

    // let img = image::open(path).unwrap();

    let mut encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let lossless_vec = encoder.encode_mem(&buf).unwrap();

    println!(
        "原始字节数: {} 压缩后字节数: {}",
        buf.len(),
        lossless_vec.len()
    );
    // fs::write(Path::new(&workspace_root).join("assets/compress/test.png"), buf.into_inner()).unwrap();
}

/// 测试双重压缩策略
///
/// 先使用 OxiPng 进行无损压缩，再使用 ImageQuant 进行有损压缩。
/// 这种组合策略可以在保持较好图像质量的同时获得更高的压缩率。
/// ImageQuant 使用 max_quality=70 的配置进行颜色量化。
#[test]
fn double_compress() {
    let buf = fs::read(get_workspace_file_path("assets/image/png/测试.png")).unwrap();

    // 无损压缩
    // let img = image::open(path).unwrap();
    let mut encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let lossless_vec = encoder.encode_mem(&buf).unwrap();

    // 有损压缩
    let mut encoder = ImageQuantEncoder::new_with_options(ImageQuantOptions {
        max_quality: 70,
        ..ImageQuantOptions::default()
    });
    let lossy_vec = encoder.encode_mem(&lossless_vec).unwrap();

    println!(
        "原始字节数: {} 压缩后字节数: {}",
        buf.len(),
        lossy_vec.len()
    );
}

/// 测试 u8 位深度图像的压缩效果
///
/// 创建 100x100 的 RGB u8 测试图像，先使用标准 PNG 编码，
/// 再使用 OxiPng 最大压缩进行优化，比较压缩前后的字节数差异。
#[test]
fn compress_u8() {
    // 246 字节
    let image = create_test_image_u8(100, 100, ColorSpace::RGB);
    let mut buf = Cursor::new(vec![]);
    let write_len = image.encode(ImageFormat::PNG, &mut buf).unwrap();
    println!("压缩前字节数: {}", write_len);

    let encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let mut buf = Cursor::new(vec![]);
    let byte_len = image.write_with_encoder(encoder, &mut buf).unwrap();

    println!("原始字节数: {} 压缩后字节数: {}", write_len, byte_len);
}

/// 测试 u8 位深度下所有支持的色彩空间编码
///
/// 遍历 OxiPng 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u8 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u8() {
    let mut results = vec![];

    let encoder = OxiPngEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u8(200, 200, *colorspace);

                let mut encoder = OxiPngEncoder::new();

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
/// 遍历 OxiPng 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 u16 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_u16() {
    let mut results = vec![];

    let encoder = OxiPngEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = OxiPngEncoder::new();

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
/// 遍历 OxiPng 编码器支持的所有色彩空间，为每个色彩空间创建 200x200 的 f32 测试图像，
/// 并验证编码是否成功。使用多线程并行测试以提高效率。
#[test]
fn encode_colorspaces_f32() {
    let mut results = vec![];

    let encoder = OxiPngEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = OxiPngEncoder::new();

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
    let mut encoder = OxiPngEncoder::new();

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
    let mut encoder = OxiPngEncoder::new();

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
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

/// 测试动画 PNG 编码
///
/// 创建包含多帧的 200x200 RGB 动画图像，验证 APNG（动画 PNG）编码功能是否正常工作。
#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
