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

#[test]
fn encode_mem_jpeg() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_workspace_file_path("assets/image/jpeg/测试-已压缩.jpg");
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

#[test]
fn encode_u8() {
    let image = create_test_image_u8(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = MozJpegEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
