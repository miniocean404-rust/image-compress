//! 统一的错误处理模块
//!
//! 使用 `thiserror` 定义项目级错误类型，提供清晰的错误分类和上下文信息。

use std::io;
use thiserror::Error;
use zune_image::errors::ImageErrors;

/// 便捷的 Result 类型别名
pub type Result<T> = std::result::Result<T, CompressError>;

/// 图像压缩核心库的统一错误类型
#[derive(Debug, Error)]
pub enum CompressError {
    // ========================================================================
    // 编解码错误
    // ========================================================================
    /// PNG 编码错误
    #[error("PNG 编码失败: {0}")]
    PngEncode(String),

    /// PNG 解码错误
    #[error("PNG 解码失败: {0}")]
    PngDecode(String),

    /// JPEG 编码错误
    #[error("JPEG 编码失败: {0}")]
    JpegEncode(String),

    /// JPEG 解码错误
    #[error("JPEG 解码失败: {0}")]
    JpegDecode(String),

    /// WebP 编码错误
    #[error("WebP 编码失败: {0}")]
    WebpEncode(String),

    /// WebP 解码错误
    #[error("WebP 解码失败: {0}")]
    WebpDecode(String),

    /// GIF 编码错误
    #[error("GIF 编码失败: {0}")]
    GifEncode(String),

    /// GIF 解码错误
    #[error("GIF 解码失败: {0}")]
    GifDecode(String),

    /// AVIF 编码错误
    #[error("AVIF 编码失败: {0}")]
    AvifEncode(String),

    /// AVIF 解码错误
    #[error("AVIF 解码失败: {0}")]
    AvifDecode(String),

    /// TIFF 解码错误
    #[error("TIFF 解码失败: {0}")]
    TiffDecode(String),

    // ========================================================================
    // 图像操作错误
    // ========================================================================
    /// 格式转换错误
    #[error("格式转换失败: {0}")]
    Transform(String),

    /// 图像缩放错误
    #[error("图像缩放失败: {0}")]
    Resize(String),

    /// ICC 配置文件处理错误
    #[error("ICC 配置文件处理失败: {0}")]
    Icc(String),

    /// 颜色量化错误
    #[error("颜色量化失败: {0}")]
    Quantize(String),

    // ========================================================================
    // 通用错误
    // ========================================================================
    /// 不支持的图像格式
    #[error("不支持的图像格式: {0}")]
    UnsupportedFormat(String),

    /// 不支持的色彩空间
    #[error("不支持的色彩空间: {0}")]
    UnsupportedColorspace(String),

    /// 无效的图像数据
    #[error("无效的图像数据: {0}")]
    InvalidData(String),

    /// 无效的参数
    #[error("无效的参数: {0}")]
    InvalidParameter(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),

    /// 来自 zune_image 的错误
    #[error("图像处理错误: {0}")]
    Image(#[from] ImageErrors),

    /// 来自 image crate 的错误
    #[error("图像处理错误: {0}")]
    ImageCrate(#[from] image::ImageError),

    /// 其他错误
    #[error("{0}")]
    Other(String),
}

// ============================================================================
// 错误转换实现
// ============================================================================

impl From<std::ffi::NulError> for CompressError {
    fn from(e: std::ffi::NulError) -> Self {
        CompressError::InvalidData(e.to_string())
    }
}

// ============================================================================
// 显式构造方法（替代过于宽松的 From 实现）
// ============================================================================

impl CompressError {
    // ========================================================================
    // 编解码错误构造方法
    // ========================================================================

    /// 创建 PNG 编码错误
    pub fn png_encode(e: impl ToString) -> Self {
        CompressError::PngEncode(e.to_string())
    }

    /// 创建 PNG 解码错误
    pub fn png_decode(e: impl ToString) -> Self {
        CompressError::PngDecode(e.to_string())
    }

    /// 创建 JPEG 编码错误
    pub fn jpeg_encode(e: impl ToString) -> Self {
        CompressError::JpegEncode(e.to_string())
    }

    /// 创建 JPEG 解码错误
    pub fn jpeg_decode(e: impl ToString) -> Self {
        CompressError::JpegDecode(e.to_string())
    }

    /// 创建 WebP 编码错误
    pub fn webp_encode(e: impl ToString) -> Self {
        CompressError::WebpEncode(e.to_string())
    }

    /// 创建 WebP 解码错误
    pub fn webp_decode(e: impl ToString) -> Self {
        CompressError::WebpDecode(e.to_string())
    }

    /// 创建 GIF 编码错误
    pub fn gif_encode(e: impl ToString) -> Self {
        CompressError::GifEncode(e.to_string())
    }

    /// 创建 GIF 解码错误
    pub fn gif_decode(e: impl ToString) -> Self {
        CompressError::GifDecode(e.to_string())
    }

    /// 创建 AVIF 编码错误
    pub fn avif_encode(e: impl ToString) -> Self {
        CompressError::AvifEncode(e.to_string())
    }

    /// 创建 AVIF 解码错误
    pub fn avif_decode(e: impl ToString) -> Self {
        CompressError::AvifDecode(e.to_string())
    }

    /// 创建 TIFF 解码错误
    pub fn tiff_decode(e: impl ToString) -> Self {
        CompressError::TiffDecode(e.to_string())
    }

    // ========================================================================
    // 图像操作错误构造方法
    // ========================================================================

    /// 创建格式转换错误
    pub fn transform(e: impl ToString) -> Self {
        CompressError::Transform(e.to_string())
    }

    /// 创建图像缩放错误
    pub fn resize(e: impl ToString) -> Self {
        CompressError::Resize(e.to_string())
    }

    /// 创建 ICC 配置文件处理错误
    pub fn icc(e: impl ToString) -> Self {
        CompressError::Icc(e.to_string())
    }

    /// 创建颜色量化错误
    pub fn quantize(e: impl ToString) -> Self {
        CompressError::Quantize(e.to_string())
    }

    // ========================================================================
    // 通用错误构造方法
    // ========================================================================

    /// 创建不支持的图像格式错误
    pub fn unsupported_format(e: impl ToString) -> Self {
        CompressError::UnsupportedFormat(e.to_string())
    }

    /// 创建不支持的色彩空间错误
    pub fn unsupported_colorspace(e: impl ToString) -> Self {
        CompressError::UnsupportedColorspace(e.to_string())
    }

    /// 创建无效的图像数据错误
    pub fn invalid_data(e: impl ToString) -> Self {
        CompressError::InvalidData(e.to_string())
    }

    /// 创建无效的参数错误
    pub fn invalid_parameter(e: impl ToString) -> Self {
        CompressError::InvalidParameter(e.to_string())
    }

    /// 创建一个通用错误
    ///
    /// 用于不属于特定分类的错误情况。优先使用具体的错误变体。
    pub fn other(msg: impl Into<String>) -> Self {
        CompressError::Other(msg.into())
    }
}

// ============================================================================
// 转换为 zune_image 错误（用于 trait 实现）
// ============================================================================

impl From<CompressError> for ImageErrors {
    fn from(e: CompressError) -> Self {
        use zune_image::errors::ImgEncodeErrors;
        ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string()))
    }
}

// ============================================================================
// 错误上下文扩展
// ============================================================================

/// 为 Result 添加上下文信息的扩展 trait
pub trait ResultExt<T> {
    /// 添加上下文信息到错误
    fn context(self, msg: &str) -> Result<T>;

    /// 使用闭包添加上下文信息
    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for std::result::Result<T, E> {
    fn context(self, msg: &str) -> Result<T> {
        self.map_err(|e| CompressError::Other(format!("{}: {}", msg, e)))
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        self.map_err(|e| CompressError::Other(format!("{}: {}", f(), e)))
    }
}

impl<T> ResultExt<T> for Option<T> {
    fn context(self, msg: &str) -> Result<T> {
        self.ok_or_else(|| CompressError::Other(msg.to_string()))
    }

    fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T> {
        self.ok_or_else(|| CompressError::Other(f()))
    }
}
