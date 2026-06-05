//! 图像格式转换测试模块
//!
//! 本模块测试图像格式转换功能，支持在不同图像格式之间进行转换。
//! 例如：PNG 转 JPEG、WebP 转 PNG 等。

#![allow(unused_imports)]
mod utils;

use std::fs;

use image::ImageFormat;
use image_compress_core::operations::transform::ImageFormatTransform;
use utils::mock::*;
use utils::path::*;

/// 测试图像格式转换功能
///
/// 从文件读取 PNG 图像，将其转换为 JPEG 格式并保存。
/// 此测试验证格式转换操作是否能正确执行，包括色彩空间和编码格式的转换。
#[test]
fn transform() {
    let file_path = get_workspace_file_path("assets/image/png/测试.png");
    let buffer = fs::read(file_path).unwrap();

    let mut transform = ImageFormatTransform::new(buffer, ImageFormat::Jpeg).unwrap();
    transform.transform().unwrap();

    fs::write("./test.jpg", transform.after).unwrap();
}
