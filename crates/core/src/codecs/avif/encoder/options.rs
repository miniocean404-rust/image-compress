use crate::codecs::OptionsTrait;
pub use ravif::{AlphaColorMode};

pub type AvifColorSpace = ravif::ColorSpace;

/// Advanced options for AVIF encoding
#[derive(Debug, Clone, Copy)]
pub struct AvifOptions {
    /// 质量 `1..=100`
    pub quality: f32,

    /// Alpha 通道的独立质量 `1..=100`
    pub alpha_quality: Option<f32>,

    /// 压缩速度 (effort) `1..=10`
    ///
    /// 1 = 非常非常慢，但是最大的压缩
    /// 10 = 快速，但文件大小较大，质量较差。
    pub speed: u8,

    /// 更改图像中颜色通道的存储方式。
    ///
    /// 请注意，这只是AVIF文件的内部细节，不会改变编码函数输入的颜色空间。
    pub color_space: AvifColorSpace,

    /// 配置透明图像中颜色通道的处理
    pub alpha_color_mode: AlphaColorMode,
}

impl OptionsTrait for AvifOptions {}
