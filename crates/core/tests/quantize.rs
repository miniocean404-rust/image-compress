//! 颜色量化操作测试模块
//!
//! 本模块测试图像颜色量化功能，包括：
//! - 基本颜色量化（减少颜色数量）
//! - 抖动（dithering）处理

mod utils;

use utils::mock::*;

use image_compress_core::operations::quantize::Quantize;
use zune_core::colorspace::ColorSpace;
use zune_image::traits::OperationsTrait;

/// 测试基本颜色量化功能
///
/// 创建 200x200 的 RGBA u8 测试图像，使用质量参数 75 进行颜色量化。
/// 颜色量化通过减少图像中的颜色数量来实现压缩，适用于 PNG 和 GIF 等格式。
#[test]
fn quantize_u8() {
    let quantize = Quantize::new(75, None);
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    assert!(result.is_ok());
}

/// 测试带抖动的颜色量化功能
///
/// 创建 200x200 的 RGBA u8 测试图像，使用质量参数 75 和抖动系数 0.75 进行量化。
/// 抖动（dithering）通过在相邻像素间混合颜色来模拟更多颜色，
/// 可以在减少颜色数量的同时保持更好的视觉效果，减少色带现象。
#[test]
fn dither_u8() {
    let quantize = Quantize::new(75, Some(0.75));
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    assert!(result.is_ok());
}
