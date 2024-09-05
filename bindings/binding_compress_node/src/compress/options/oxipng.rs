use std::time::Duration;

use image_compress::export::{self, IndexSet};
use napi::bindgen_prelude::{Array, BigInt, Object};
use napi_derive::napi;

#[napi(object, js_name = "OxiPngOptions")]
pub struct NapiOxiPngOptions {
    /// fix_errors: bool:               尝试在解码输入文件时修复错误，而不是返回 `Err`。默认值: `false`
    pub fix_errors: bool,

    /// force: bool:                    即使压缩没有改进，也写入输出。默认值: `false`
    pub force: bool,

    /// filter: IndexSet<RowFilter>:    尝试在文件上使用哪些 RowFilters。默认值: `None,Sub,Entropy,Bigrams`
    pub filter: RowFilter,

    /// interlace: Option<Interlacing>: 是否更改文件的交错类型。`None` 将不会更改当前的交错类型。`Some(x)` 将把文件更改为交错模式 `x`。默认值: `Some(Interlacing::None)`
    pub interlace: Option<Interlacing>,

    /// optimize_alpha: bool:           是否允许更改透明像素以提高压缩率。
    pub optimize_alpha: bool,

    /// bit_depth_reduction: bool:      是否尝试位深度减少。默认值: `true`
    pub bit_depth_reduction: bool,

    /// color_type_reduction: bool:     是否尝试颜色类型减少。默认值: `true`
    pub color_type_reduction: bool,

    /// palette_reduction: bool:        是否尝试调色板减少。默认值: `true`
    pub palette_reduction: bool,

    /// grayscale_reduction: bool:      是否尝试灰度减少。默认值: `true`
    pub grayscale_reduction: bool,

    /// idat_recoding: bool:            是否对 IDAT 和其他压缩块进行重新编码。如果执行任何类型的减少，将无视此设置执行 IDAT 重新编码。默认值: `true`
    pub idat_recoding: bool,

    /// scale_16: bool:                 是否强制将 16 位缩减为 8 位。默认值: `false`
    pub scale_16: bool,

    /// strip: StripChunks:             从 PNG 文件中剥离哪些块（如果有的话）。默认值: `None`
    pub strip: StripChunks,

    /// deflate: Deflaters:             使用哪种 DEFLATE 算法。默认值: `Libdeflater`
    pub deflate: Deflaters,

    /// fast_evaluation: bool:          是否使用快速评估来选择最佳过滤器。默认值: `true`
    pub fast_evaluation: bool,

    /// timeout: Option<Duration>:      优化的最大时间。如果超时，将跳过进一步的潜在优化。
    pub timeout: Option<BigInt>,
    // 仅供参考的 demo
    // pub date: Option<chrono::DateTime<Utc>>,
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

// ! 未写全
impl From<RowFilter> for export::IndexSet<export::RowFilter> {
    fn from(value: RowFilter) -> Self {
        match value {
            RowFilter::None => IndexSet::new(),
            RowFilter::Sub => IndexSet::new(),
            RowFilter::Up => IndexSet::new(),
            RowFilter::Average => IndexSet::new(),
            RowFilter::Paeth => IndexSet::new(),
            RowFilter::MinSum => IndexSet::new(),
            RowFilter::Entropy => IndexSet::new(),
            RowFilter::Bigrams => IndexSet::new(),
            RowFilter::BigEnt => IndexSet::new(),
            RowFilter::Brute => IndexSet::new(),
        }
    }
}

#[napi(string_enum)]
pub enum Interlacing {
    None,
    Adam7,
}

impl From<Interlacing> for export::Interlacing {
    fn from(value: Interlacing) -> Self {
        match value {
            Interlacing::None => export::Interlacing::None,
            Interlacing::Adam7 => export::Interlacing::Adam7,
        }
    }
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

impl From<Deflaters> for export::Deflaters {
    fn from(value: Deflaters) -> Self {
        match value {
            Deflaters::Libdeflater { compression } => {
                export::Deflaters::Libdeflater { compression }
            }
        }
    }
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

// ! 未写全
impl From<StripChunks> for export::StripChunks {
    fn from(value: StripChunks) -> Self {
        match value {
            StripChunks::None => export::StripChunks::None,
            StripChunks::Strip(_) => export::StripChunks::Strip(IndexSet::new()),
            StripChunks::Safe => export::StripChunks::Safe,
            StripChunks::Keep(_) => export::StripChunks::Keep(IndexSet::new()),
            StripChunks::All => export::StripChunks::All,
        }
    }
}

impl From<NapiOxiPngOptions> for export::OxiPngOptions {
    fn from(value: NapiOxiPngOptions) -> Self {
        let (_signed, timeout, _is_lossless) = value.timeout.unwrap().get_u64();

        export::OxiPngOptions {
            fix_errors: value.fix_errors,
            force: value.force,
            filter: value.filter.into(),
            interlace: value.interlace.map(|x| x.into()),
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
            timeout: Some(Duration::from_millis(timeout)),
        }
    }
}

impl From<Object> for NapiOxiPngOptions {
    fn from(value: Object) -> Self {
        Self {
            fix_errors: value.get_named_property::<bool>("fixErrors").unwrap(),
            force: value.get_named_property::<bool>("force").unwrap(),
            filter: value.get_named_property::<RowFilter>("filter").unwrap(),
            interlace: value
                .get_named_property::<Option<Interlacing>>("interlace")
                .unwrap(),
            optimize_alpha: value.get_named_property::<bool>("optimizeAlpha").unwrap(),
            bit_depth_reduction: value
                .get_named_property::<bool>("bitDepthReduction")
                .unwrap(),
            color_type_reduction: value
                .get_named_property::<bool>("colorTypeReduction")
                .unwrap(),
            palette_reduction: value
                .get_named_property::<bool>("paletteReduction")
                .unwrap(),
            grayscale_reduction: value
                .get_named_property::<bool>("grayscaleReduction")
                .unwrap(),
            idat_recoding: value.get_named_property::<bool>("idatRecoding").unwrap(),
            scale_16: value.get_named_property::<bool>("scale16").unwrap(),
            strip: value.get_named_property::<StripChunks>("strip").unwrap(),
            deflate: value.get_named_property::<Deflaters>("deflate").unwrap(),
            fast_evaluation: value.get_named_property::<bool>("fastEvaluation").unwrap(),
            timeout: value
                .get_named_property::<Option<BigInt>>("timeout")
                .unwrap(),
        }
    }
}
