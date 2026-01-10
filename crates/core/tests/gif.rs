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

#[test]
fn gif_options_default() {
    let options = GifOptions::default();
    assert_eq!(options.lossy, 0);
    assert_eq!(options.optimize_level, 2);
    assert!(!options.reduce_colors);
    assert_eq!(options.max_colors, 256);
}

#[test]
fn gif_options_lossless() {
    let options = GifOptions::lossless();
    assert_eq!(options.lossy, 0);
    assert_eq!(options.optimize_level, 3);
}

#[test]
fn gif_options_lossy() {
    let options = GifOptions::lossy(50);
    assert_eq!(options.lossy, 100); // (100 - 50) * 2 = 100
}

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
