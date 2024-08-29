#![allow(unused_imports)]
mod utils;

use utils::mock::*;
use utils::path::*;

use cargo_metadata::MetadataCommand;
use image_compress_core::codecs::png::imagequant::{ImageQuantEncoder, ImageQuantOptions};
use image_compress_core::codecs::png::oxipng::OxiPngEncoder;
use std::path::{Path, PathBuf};
use std::{fs, io::Cursor};
use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::ImageFormat;
use zune_image::image::Image;

use image::{DynamicImage, ImageBuffer, Rgba};

#[test]
fn compress_u8_new() {
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

    // let metadata = MetadataCommand::new()
    //     .exec()
    //     .unwrap();
    // let workspace_root = metadata.workspace_root;
    // let path = Path::new(&workspace_root).join("assets/image/png/time-icon.png");
    // let image = Image::open(&path).unwrap();

    let encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let mut buf = Cursor::new(vec![]);
    let byte_len = image.write_with_encoder(encoder, &mut buf).unwrap();

    println!("原始字节数: {} 压缩后字节数: {}", write_len, byte_len);
    // fs::write(Path::new(&workspace_root).join("assets/compress/test.png"), buf.into_inner()).unwrap();
}

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
