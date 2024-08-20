#![allow(unused_imports)]

use std::{fs, io::Cursor, path::Path};

use cargo_metadata::MetadataCommand;
use image::{DynamicImage, ImageBuffer, Rgba};
use image_compress_core::png::codec::oxipng::OxiPngEncoder;

#[test]
fn compress_u8_demo() {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let path = Path::new(&workspace_root).join("assets/image/png/time-icon.png");
    let buf = fs::read(path).unwrap();
    println!("原始字节数: {}", buf.len());

    let img = image::load_from_memory(&buf).unwrap();
    // let img = image::open(path).unwrap();

    let encoder = OxiPngEncoder::new_with_options(oxipng::Options::max_compression());
    let result = encoder.encode(&img).unwrap();
    println!("压缩后字节数: {}", result.len());

    // fs::write(Path::new(&workspace_root).join("assets/compress/test.png"), buf.into_inner()).unwrap();
}
