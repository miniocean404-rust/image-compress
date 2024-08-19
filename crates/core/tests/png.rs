#![allow(unused_imports)]
mod mock;

use cargo_metadata::MetadataCommand;
use image_compress_core::png::oxipng_lossless::OxiPngEncoder;
use mock::create_test_image_u8;
use std::path::Path;
use std::{fs, io::Cursor};
use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::ImageFormat;
use zune_image::image::Image;

#[test]
fn compress_u8() {
    // 246 字节
    let image = create_test_image_u8(100, 100, ColorSpace::RGB);
    let mut buf = Cursor::new(vec![]);
    let write = image.encode(ImageFormat::PNG, &mut buf).unwrap();
    println!("压缩前字节数: {}", write);

    // let metadata = MetadataCommand::new()
    //     .exec()
    //     .unwrap();
    // let workspace_root = metadata.workspace_root;
    // let path = Path::new(&workspace_root).join("assets/image/png/time-icon.png");
    // let image = Image::open(&path).unwrap();

    let encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let mut buf = Cursor::new(vec![]);
    let byte_len = image.write_with_encoder(encoder, &mut buf).unwrap();

    println!("压缩后字节数: {}", byte_len);

    // fs::write(Path::new(&workspace_root).join("assets/compress/test.png"), buf.into_inner()).unwrap();
}
