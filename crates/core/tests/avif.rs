use std::{
    fs::{self},
    io::{BufReader, Cursor},
};
mod utils;

use image_compress_core::codecs::avif::{decoder::AvifDecoder, encoder::AvifEncoder};
use utils::{mock::*, path::get_workspace_file_path};
use zune_core::colorspace::ColorSpace;
use zune_image::{image::Image, traits::EncoderTrait};

#[test]
fn encode_mem_avif() -> Result<(), Box<dyn std::error::Error>> {
    let byte_vec = fs::read(get_workspace_file_path("assets/image/avif/f1t.avif"))?;
    let cursor = Cursor::new(&byte_vec);
    let reader = BufReader::new(cursor);

    let decoder = AvifDecoder::try_new(reader)?;
    let image = Image::from_decoder(decoder)?;

    let mut compress_buf = Cursor::new(vec![]);
    let mut encoder = AvifEncoder::new();

    encoder.encode(&image, &mut compress_buf)?;

    println!(
        "原始字节数: {} 压缩后字节数: {}",
        byte_vec.len(),
        compress_buf.get_ref().len()
    );

    Ok(())
}

#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let buf = fs::read(get_workspace_file_path("assets/image/avif/f1t.avif"))?;
    let cursor = Cursor::new(&buf);
    let decoder = AvifDecoder::try_new(cursor)?;

    let image = Image::from_decoder(decoder)?;

    assert_eq!(image.dimensions(), (48, 80));
    assert_eq!(image.colorspace(), ColorSpace::RGBA);
    Ok(())
}

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

    let encoder = AvifEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_u16(200, 200, *colorspace);

                let mut encoder = AvifEncoder::new();

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

    let encoder = AvifEncoder::new();

    for colorspace in encoder.supported_colorspaces() {
        let builder = std::thread::Builder::new().name(format!("{:?}", colorspace));

        let handler = builder
            .spawn(move || {
                let image = create_test_image_f32(200, 200, *colorspace);

                let mut encoder = AvifEncoder::new();

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
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_u16() {
    let image = create_test_image_u16(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_f32() {
    let image = create_test_image_f32(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn encode_animated() {
    let image = create_test_image_animated(200, 200, ColorSpace::RGB);
    let mut encoder = AvifEncoder::new();

    let buf = Cursor::new(vec![]);

    let result = encoder.encode(&image, buf);
    dbg!(&result);

    assert!(result.is_ok());
}
