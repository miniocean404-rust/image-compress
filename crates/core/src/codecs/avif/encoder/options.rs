pub use ravif::AlphaColorMode;

pub type AvifColorSpace = ravif::ColorModel;

/// AVIF 编码高级选项
///
/// AVIF 是基于 AV1 视频编解码器的现代图像格式，提供卓越的压缩效率。
/// 在相同视觉质量下，AVIF 通常比 JPEG 小 50%，比 WebP 小 20%。
#[derive(Debug, Clone, Copy)]
pub struct AvifOptions {
    /// 质量 `1..=100`
    ///
    /// - 100: 无损压缩
    /// - 80-90: 高质量，人眼几乎无法察觉差异（推荐用于照片）
    /// - 60-80: 良好质量，轻微可见差异（推荐用于网页）
    /// - 40-60: 中等质量，明显压缩痕迹
    /// - < 40: 低质量，明显失真
    ///
    /// 默认值: 80（在人眼难以察觉的前提下最大化压缩）
    pub quality: f32,

    /// Alpha 通道的独立质量 `1..=100`
    ///
    /// Alpha 通道对视觉影响较小，可以使用较低质量以获得更好压缩。
    /// 设置为 None 时使用与主质量相同的值。
    ///
    /// 默认值: None（跟随主质量）
    pub alpha_quality: Option<f32>,

    /// 压缩速度 (effort) `1..=10`
    ///
    /// - 1: 最慢，最大压缩率（适合最终发布）
    /// - 4: 平衡速度和压缩率（推荐）
    /// - 6: 较快，适合批量处理
    /// - 10: 最快，文件较大
    ///
    /// 默认值: 4（平衡模式）
    pub speed: u8,

    /// 内部颜色模型
    ///
    /// - YCbCr: 标准模式，适合大多数图像（推荐）
    /// - RGB: 保留原始 RGB 值，适合需要精确颜色的场景
    ///
    /// 请注意，这只是 AVIF 文件的内部细节，不会改变编码函数输入的颜色空间。
    pub color_space: AvifColorSpace,

    /// 透明图像中颜色通道的处理模式
    ///
    /// - UnassociatedClean: 清理透明区域的颜色值，提高压缩率（推荐）
    /// - UnassociatedDirty: 保留透明区域的原始颜色值
    /// - Premultiplied: 预乘 Alpha
    pub alpha_color_mode: AlphaColorMode,
}
