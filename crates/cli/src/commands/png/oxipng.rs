use clap::Parser;

use crate::commands::CommandRunner;

#[derive(Parser, Debug)]
pub struct OxipngOptions {
    /// 尝试在解码输入文件时修复错误，而不是返回 `Err`。
    ///
    /// 默认值: `false`
    #[arg(long, default_value_t = false)]
    fix_errors: bool,

    /// 即使压缩没有改进，也写入输出。
    ///
    /// 默认值: `false`
    #[arg(long, default_value_t = false)]
    force: bool,

    /// 尝试在文件上使用哪些 RowFilters
    ///
    /// 默认值: `None,Sub,Entropy,Bigrams`
    // #[arg(long)]
    // filter: Option<RowFilter>,

    /// 是否更改文件的交错类型。
    ///
    /// `None` 将不会更改当前的交错类型。
    ///
    /// `Some(x)` 将把文件更改为交错模式 `x`。
    ///
    /// 默认值: `Some(Interlacing::None)`
    // #[arg(long)]
    // interlace: Option<Interlacing>,

    /// 是否允许更改透明像素以提高压缩率。
    #[arg(long)]
    optimize_alpha: bool,

    /// 是否尝试减少位深度
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    bit_depth_reduction: bool,

    /// 是否尝试减少颜色类型
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    color_type_reduction: bool,

    /// 是否尝试减少调色板
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    palette_reduction: bool,

    /// 是否尝试减少灰度
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    grayscale_reduction: bool,

    /// 是否对 IDAT 和其他压缩块进行重新编码
    ///
    /// 如果执行任何类型的减少操作，将无论此设置如何都执行 IDAT 重新编码
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    idat_recoding: bool,

    /// 是否通过缩放强制将 16 位减少到 8 位
    ///
    /// 默认值: `false`
    #[arg(long, default_value_t = false)]
    scale_16: bool,

    /// 从 PNG 文件中剥离哪些块（如果有）
    ///
    /// 默认值: `None`
    // #[arg(long)]
    // strip: Option<StripChunks>,

    /// 使用哪种 DEFLATE 算法
    ///
    /// 默认值: `Libdeflater`
    // #[arg(long)]
    // deflate: Option<Deflaters>,

    /// 是否使用快速评估来选择最佳过滤器
    ///
    /// 默认值: `true`
    #[arg(long, default_value_t = true)]
    fast_evaluation: bool,
    ///// 用于优化的最大时间。
    ///// 如果超时，将跳过进一步的潜在优化。
    // #[arg(long)]
    // timeout: Option<Duration>,
}

impl CommandRunner for OxipngOptions {
    fn execute(&self) -> anyhow::Result<()> {
        println!("OxipngOptions");

        Ok(())
    }
}
