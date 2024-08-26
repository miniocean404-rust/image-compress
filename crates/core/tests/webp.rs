#![allow(unused_imports)]
mod utils;

use utils::mock::*;
use utils::path::*;

use std::fs;
use std::fs::File;
use std::io::Cursor;

use image_compress_core::webp::codec::decoder::WebPDecoder;
use image_compress_core::webp::codec::encoder::WebPEncoder;
use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_image::image::Image;
use zune_image::traits::EncoderTrait;

#[test]
fn encode_mem_webp() -> Result<(), Box<dyn std::error::Error>> {
    let buf = fs::read(get_workspace_file_path("assets/image/webp/time-icon.webp"))?;
    // let buffer = Cursor::new(&buf);

    let file_content = File::open(get_workspace_file_path("assets/image/webp/time-icon.webp"))?;
    let decoder = WebPDecoder::try_new(file_content)?;
    let image = Image::from_decoder(decoder)?;

    let compress_buf = Cursor::new(vec![]);
    let mut encoder = WebPEncoder::new();

    let result = encoder.encode(&image, compress_buf)?;
    println!("原始字节数: {} 压缩后字节数: {}", buf.len(), result);

    Ok(())
}

#[test]
fn decode() {
    let file_content = File::open("tests/files/webp/f1t.webp").unwrap();

    let decoder = WebPDecoder::try_new(file_content).unwrap();

    let img = Image::from_decoder(decoder).unwrap();

    assert_eq!(img.dimensions(), (48, 80));
    assert_eq!(img.colorspace(), ColorSpace::RGBA);
}

#[test]
fn encode_colorspaces_u8() {
    let mut results = vec![];

    let encoder = WebPEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u8(200, 200, *colorspace);

                let mut encoder = WebPEncoder::new();

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

    let encoder = WebPEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = WebPEncoder::new();

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

    let encoder = WebPEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = WebPEncoder::new();

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
    let mut encoder = WebPEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = WebPEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = WebPEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = WebPEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
