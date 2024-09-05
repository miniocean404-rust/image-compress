pub use oxipng::{Deflaters, IndexSet, Interlacing, RowFilter, StripChunks};

/// [`oxipng::Options`] 别名
/// fix_errors: bool:               尝试在解码输入文件时修复错误，而不是返回 `Err`。默认值: `false`
/// force: bool:                    即使压缩没有改进，也写入输出。默认值: `false`
/// filter: IndexSet<RowFilter>:    尝试在文件上使用哪些 RowFilters。默认值: `None,Sub,Entropy,Bigrams`
/// interlace: Option<Interlacing>: 是否更改文件的交错类型。`None` 将不会更改当前的交错类型。`Some(x)` 将把文件更改为交错模式 `x`。默认值: `Some(Interlacing::None)`
/// optimize_alpha: bool:           是否允许更改透明像素以提高压缩率。
/// bit_depth_reduction: bool:      是否尝试位深度减少。默认值: `true`
/// color_type_reduction: bool:     是否尝试颜色类型减少。默认值: `true`
/// palette_reduction: bool:        是否尝试调色板减少。默认值: `true`
/// grayscale_reduction: bool:      是否尝试灰度减少。默认值: `true`
/// idat_recoding: bool:            是否对 IDAT 和其他压缩块进行重新编码。如果执行任何类型的减少，将无视此设置执行 IDAT 重新编码。默认值: `true`
/// scale_16: bool:                 是否强制将 16 位缩减为 8 位。默认值: `false`
/// strip: StripChunks:             从 PNG 文件中剥离哪些块（如果有的话）。默认值: `None`
/// deflate: Deflaters:             使用哪种 DEFLATE 算法。默认值: `Libdeflater`
/// fast_evaluation: bool:          是否使用快速评估来选择最佳过滤器。默认值: `true`
/// timeout: Option<Duration>:      优化的最大时间。如果超时，将跳过进一步的潜在优化。
pub type OxiPngOptions = oxipng::Options;
