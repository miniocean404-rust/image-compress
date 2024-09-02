use chrono::Utc;
use napi::bindgen_prelude::Array;
use napi_derive::napi;
use image_compress::export;


#[napi(object)]
pub struct OxiPngOptions {
    /// Attempt to fix errors when decoding the input file rather than returning an `Err`.
    ///
    /// Default: `false`
    pub fix_errors: bool,
    /// Write to output even if there was no improvement in compression.
    ///
    /// Default: `false`
    pub force: bool,
    /// Which RowFilters to try on the file
    ///
    /// Default: `None,Sub,Entropy,Bigrams`
    pub filter: RowFilter,
    /// Whether to change the interlacing type of the file.
    ///
    /// `None` will not change the current interlacing type.
    ///
    /// `Some(x)` will change the file to interlacing mode `x`.
    ///
    /// Default: `Some(Interlacing::None)`
    pub interlace: Option<Interlacing>,
    /// Whether to allow transparent pixels to be altered to improve compression.
    pub optimize_alpha: bool,
    /// Whether to attempt bit depth reduction
    ///
    /// Default: `true`
    pub bit_depth_reduction: bool,
    /// Whether to attempt color type reduction
    ///
    /// Default: `true`
    pub color_type_reduction: bool,
    /// Whether to attempt palette reduction
    ///
    /// Default: `true`
    pub palette_reduction: bool,
    /// Whether to attempt grayscale reduction
    ///
    /// Default: `true`
    pub grayscale_reduction: bool,
    /// Whether to perform recoding of IDAT and other compressed chunks
    ///
    /// If any type of reduction is performed, IDAT recoding will be performed
    /// regardless of this setting
    ///
    /// Default: `true`
    pub idat_recoding: bool,
    /// Whether to forcibly reduce 16-bit to 8-bit by scaling
    ///
    /// Default: `false`
    pub scale_16: bool,
    /// Which chunks to strip from the PNG file, if any
    ///
    /// Default: `None`
    pub strip: StripChunks,
    /// Which DEFLATE algorithm to use
    ///
    /// Default: `Libdeflater`
    pub deflate: Deflaters,
    /// Whether to use fast evaluation to pick the best filter
    ///
    /// Default: `true`
    pub fast_evaluation: bool,

    /// Maximum amount of time to spend on optimizations.
    /// Further potential optimizations are skipped if the timeout is exceeded.
    pub timeout: Option<chrono::DateTime<Utc>>,
}


#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum RowFilter {
    // 标准过滤器类型
    None,
    Sub,
    Up,
    Average,
    Paeth,
    // 启发式策略
    MinSum,
    Entropy,
    Bigrams,
    BigEnt,
    Brute,
}

#[napi(string_enum)]
pub enum Interlacing {
    None,
    Adam7,
}

#[napi(discriminant = "type2")]
pub enum Deflaters {
    /// 使用 libdeflater.
    Libdeflater {
        /// 对文件使用哪个压缩级别 （1-12）
        compression: u8,
    },
    // #[cfg(feature = "zopfli")]
    // /// 使用更好但速度较慢的 Zopfli 实现
    // Zopfli {
    //     // 要执行的压缩迭代次数。15 次迭代就可以了
    //     // 对于小文件，但较大的文件需要使用
    //     // 更少的迭代，否则它们会太慢。
    //     iterations: NonZeroU8,
    // },
}


#[napi(discriminant = "type2")]
pub enum StripChunks {
    /// 无
    None,
    /// 删除特定块, 长度为 4 的数组
    Strip(Array),
    /// 删除所有不会影响图像显示的数据块
    Safe,
    /// 删除除这些之外的所有非关键块, 长度为 4 的数组
    Keep(Array),
    /// 所有非关键块
    All,
}


impl From<OxiPngOptions> for export::OxiPngOptions {
    fn from(value: OxiPngOptions) -> Self {
        export::OxiPngOptions {
            fix_errors: value.fix_errors,
            force: value.force,
            filter: value.filter.into(),
            interlace: value.interlace.into(),
            optimize_alpha: value.optimize_alpha,
            bit_depth_reduction: value.bit_depth_reduction,
            color_type_reduction: value.color_type_reduction,
            palette_reduction: value.palette_reduction,
            grayscale_reduction: value.grayscale_reduction,
            idat_recoding: value.idat_recoding,
            scale_16: value.scale_16,
            strip: value.strip.into(),
            deflate: value.deflate.into(),
            fast_evaluation: value.fast_evaluation,
            timeout: value.timeout.map(|x| x.into()),
        }
    }
}