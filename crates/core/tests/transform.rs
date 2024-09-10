#![allow(unused_imports)]
mod utils;

use std::fs;

use image::ImageFormat;
use image_compress_core::operations::transform::ImageFormatTransform;
use utils::mock::*;
use utils::path::*;

#[test]
fn transform() {
    let file_path = get_workspace_file_path("assets/image/png/little.png");
    let buffer = fs::read(file_path).unwrap();

    let mut transform = ImageFormatTransform::new(buffer, ImageFormat::Jpeg);
    transform.transform().unwrap();

    fs::write("./test.jpg", transform.after).unwrap();
}
