//! 测试图像生成模块
//!
//! 本模块提供用于测试的模拟图像生成功能，支持：
//! - 不同位深度（u8、u16、f32）的图像生成
//! - 不同色彩空间的图像生成
//! - 动画图像（多帧）的生成
//!
//! 生成的测试图像使用简单的渐变模式，红色分量随 x 坐标变化，
//! 蓝色分量随 y 坐标变化，便于视觉验证和调试。

#![allow(dead_code)]
use zune_core::{
    bit_depth::{BitDepth, BitType},
    colorspace::ColorSpace,
};
use zune_image::{channel::Channel, frame::Frame, image::Image};

/// 创建 u8 位深度的测试图像
///
/// 生成指定尺寸和色彩空间的测试图像，像素值使用渐变模式：
/// - 红色分量 = 0.3 * x
/// - 绿色分量 = 0
/// - 蓝色分量 = 0.3 * y
///
/// # 参数
///
/// * `width` - 图像宽度（像素）
/// * `height` - 图像高度（像素）
/// * `colorspace` - 目标色彩空间
///
/// # 返回值
///
/// 返回生成的测试图像
pub fn create_test_image_u8(width: usize, height: usize, colorspace: ColorSpace) -> Image {
    Image::from_fn(width, height, colorspace, |x, y, px: &mut [u8; 4]| {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;

        px[0] = r;
        px[1] = 0;
        px[2] = b;
    })
}

/// 创建 u16 位深度的测试图像
///
/// 生成指定尺寸和色彩空间的 16 位测试图像，像素值使用渐变模式：
/// - 红色分量 = 0.3 * x
/// - 绿色分量 = 0
/// - 蓝色分量 = 0.3 * y
///
/// # 参数
///
/// * `width` - 图像宽度（像素）
/// * `height` - 图像高度（像素）
/// * `colorspace` - 目标色彩空间
///
/// # 返回值
///
/// 返回生成的测试图像
pub fn create_test_image_u16(width: usize, height: usize, colorspace: ColorSpace) -> Image {
    Image::from_fn(width, height, colorspace, |x, y, px: &mut [u16; 4]| {
        let r = (0.3 * x as f32) as u16;
        let b = (0.3 * y as f32) as u16;

        px[0] = r;
        px[1] = 0;
        px[2] = b;
    })
}

/// 创建 f32 位深度的测试图像
///
/// 生成指定尺寸和色彩空间的浮点测试图像，像素值使用渐变模式：
/// - 红色分量 = 0.3 * x
/// - 绿色分量 = 0.0
/// - 蓝色分量 = 0.3 * y
///
/// # 参数
///
/// * `width` - 图像宽度（像素）
/// * `height` - 图像高度（像素）
/// * `colorspace` - 目标色彩空间
///
/// # 返回值
///
/// 返回生成的测试图像
pub fn create_test_image_f32(width: usize, height: usize, colorspace: ColorSpace) -> Image {
    Image::from_fn(width, height, colorspace, |x, y, px: &mut [f32; 4]| {
        let r = 0.3 * x as f32;
        let b = 0.3 * y as f32;

        px[0] = r;
        px[1] = 0.;
        px[2] = b;
    })
}

/// 创建动画测试图像
///
/// 生成包含 6 帧的动画测试图像，每帧使用 u8 位深度和 3 个颜色通道。
/// 用于测试编码器对多帧图像的处理能力。
///
/// # 参数
///
/// * `width` - 图像宽度（像素）
/// * `height` - 图像高度（像素）
/// * `colorspace` - 目标色彩空间
///
/// # 返回值
///
/// 返回生成的动画测试图像
pub fn create_test_image_animated(width: usize, height: usize, colorspace: ColorSpace) -> Image {
    let mut frames = vec![];

    let channel_length = width * height;

    (0..=5).for_each(|_| {
        let channels = vec![Channel::new_with_bit_type(channel_length, BitType::U8); 3];
        frames.push(Frame::new(channels))
    });

    Image::new_frames(frames, BitDepth::Eight, width, height, colorspace)
}
