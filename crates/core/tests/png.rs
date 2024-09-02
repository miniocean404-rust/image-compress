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

#[test]
fn image_quant_compress_lossy() {
    let file_path = get_workspace_file_path("assets/image/png/time-icon.png");

    let buf = fs::read(&file_path).unwrap();

    let mut encoder = ImageQuantEncoder::new();
    let lossy_vec = encoder.encode_mem(&buf).unwrap();

    println!(
        "原始字节数: {} 压缩后字节数: {}",
        buf.len(),
        lossy_vec.len()
    );
    // fs::write(Path::new(&workspace_root).join("assets/compress/test.png"), buf.into_inner()).unwrap();
}

#[test]
fn oxipng_compress_lossless() {
    let buf = fs::read(get_workspace_file_path("assets/image/png/time-icon.png")).unwrap();

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

#[test]
fn double_compress() {
    let buf = fs::read(get_workspace_file_path("assets/image/png/time-icon.png")).unwrap();

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

#[test]
fn encode_u8() {
    let image = create_test_image_u8(200, 200, ColorSpace::RGB);
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = OxiPngEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
