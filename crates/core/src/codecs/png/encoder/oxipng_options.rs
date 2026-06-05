pub use oxipng::{Deflater, FilterStrategy, IndexSet, RowFilter, StripChunks};

/// OxiPNG 无损压缩选项
///
/// OxiPNG 是一个多线程无损 PNG 压缩优化器。
/// 它通过尝试不同的过滤器和压缩参数来找到最小的文件大小，
/// 同时保持图像完全无损。
///
/// ## 主要配置项
///
/// - `fix_errors`: 尝试修复解码错误，而不是返回 `Err`。默认值: `false`
/// - `force`: 即使压缩没有改进，也写入输出。默认值: `false`
/// - `filters`: 尝试的过滤策略集合。默认值: `None, Sub, Entropy, Bigrams`
/// - `interlace`: 是否更改交错类型。`None` 不更改，`Some(false)` 关闭交错（推荐）
/// - `optimize_alpha`: 允许更改透明像素以提高压缩率。默认值: `false`
/// - `bit_depth_reduction`: 尝试位深度减少。默认值: `true`
/// - `color_type_reduction`: 尝试颜色类型减少。默认值: `true`
/// - `palette_reduction`: 尝试调色板减少。默认值: `true`
/// - `grayscale_reduction`: 尝试灰度减少。默认值: `true`
/// - `idat_recoding`: 重新编码 IDAT 块。默认值: `true`
/// - `scale_16`: 强制将 16 位缩减为 8 位（有损）。默认值: `false`
/// - `strip`: 剥离的元数据块。默认值: `None`
/// - `deflater`: DEFLATE 算法。默认值: `Libdeflater`
/// - `fast_evaluation`: 快速评估选择最佳过滤器。默认值: `true`
/// - `timeout`: 优化超时时间
/// - `max_decompressed_size`: 输入 IDAT 的最大解压大小
///
/// ## 推荐配置（最大压缩）
///
/// ```rust
/// use oxipng::{Options, Deflater, StripChunks, indexset, FilterStrategy};
///
/// let mut opts = Options::max_compression();
/// opts.strip = StripChunks::Safe;  // 移除非显示相关的元数据
/// opts.optimize_alpha = true;       // 优化透明像素
/// opts.fast_evaluation = true;      // 快速评估模式
/// ```
pub type OxiPngOptions = oxipng::Options;

/// 创建优化的 OxiPNG 配置
///
/// 此函数返回针对最大压缩优化的配置，同时保持无损质量。
pub fn create_optimized_oxipng_options() -> OxiPngOptions {
    let mut opts = oxipng::Options::max_compression();

    // 移除非显示相关的元数据块（保留颜色配置文件等重要信息）
    opts.strip = StripChunks::Safe;

    // 允许修改透明像素以提高压缩率
    // 这不会影响视觉效果，因为透明像素本身不可见
    opts.optimize_alpha = true;

    // 启用所有减少优化
    opts.bit_depth_reduction = true;
    opts.color_type_reduction = true;
    opts.palette_reduction = true;
    opts.grayscale_reduction = true;

    // 关闭交错（非交错通常压缩更好）
    opts.interlace = Some(false);

    // 启用快速评估模式
    opts.fast_evaluation = true;

    // 使用 libdeflater 最高压缩级别
    opts.deflater = Deflater::Libdeflater { compression: 12 };

    opts
}
