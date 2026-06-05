//! 图像缩放操作测试模块
//!
//! 本模块测试图像缩放功能，使用 fast_image_resize 库，包括：
//! - 不同位深度的缩放支持（u8、u16、f32）
//! - 动画图像的缩放支持
//! - 使用最近邻插值算法进行缩放

mod utils;

use utils::mock::*;

use fast_image_resize::{self as fr};
use image_compress_core::operations::resize::Resize;
use zune_core::colorspace::ColorSpace;
use zune_image::traits::OperationsTrait;

/// 测试 u8 位深度图像的缩放功能
///
/// 创建 200x200 的 RGB u8 测试图像，使用最近邻插值算法缩放到 100x100。
/// 验证缩放操作成功完成且输出尺寸正确。
#[test]
fn resize_u8() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

/// 测试 u16 位深度图像的缩放功能
///
/// 创建 200x200 的 RGB u16 测试图像，使用最近邻插值算法缩放到 100x100。
/// 验证 16 位深度图像的缩放操作成功完成且输出尺寸正确。
#[test]
fn resize_u16() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_u16(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

/// 测试 f32 位深度图像的缩放功能
///
/// 创建 200x200 的 RGB f32 测试图像，使用最近邻插值算法缩放到 100x100。
/// 验证浮点位深度图像的缩放操作成功完成且输出尺寸正确。
#[test]
fn resize_f32() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_f32(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}

/// 测试动画图像的缩放功能
///
/// 创建包含多帧的 200x200 RGB 动画图像，使用最近邻插值算法缩放到 100x100。
/// 验证动画图像的所有帧都能正确缩放且输出尺寸正确。
#[test]
fn resize_animated() {
    let resize = Resize::new(100, 100, fr::ResizeAlg::Nearest);
    let mut image = create_test_image_animated(200, 200, ColorSpace::RGB);

    let result = resize.execute(&mut image);
    dbg!(&result);

    assert!(result.is_ok());
    assert_eq!(image.dimensions(), (100, 100));
}
