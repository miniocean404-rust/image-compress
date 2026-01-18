use std::time::Duration;

use image_compress::export::{self, IndexSet};
use napi::bindgen_prelude::{Array, BigInt, JsObjectValue, Object};
use napi_derive::napi;

#[napi(object, js_name = "OxiPngOptions")]
pub struct NapiOxiPngOptions {
    /// fix_errors: bool:               尝试在解码输入文件时修复错误，而不是返回 `Err`。默认值: `false`
    pub fix_errors: bool,

    /// force: bool:                    即使压缩没有改进，也写入输出。默认值: `false`
    pub force: bool,

    /// filters: IndexSet<FilterStrategy>: 尝试在文件上使用哪些 FilterStrategy。默认值: `None,Sub,Entropy,Bigrams`
    pub filters: FilterStrategy,

    /// interlace: Option<bool>:        是否更改文件的交错类型。`None` 将不会更改当前的交错类型。`Some(true)` 开启交错，`Some(false)` 关闭交错。默认值: `Some(false)`
    pub interlace: Option<bool>,

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

    /// deflater: Deflater:             使用哪种 DEFLATE 算法。默认值: `Libdeflater`
    pub deflater: Deflater,

    /// fast_evaluation: bool:          是否使用快速评估来选择最佳过滤器。默认值: `true`
    pub fast_evaluation: bool,

    /// timeout: Option<Duration>:      优化的最大时间（毫秒）。如果超时，将跳过进一步的潜在优化。
    pub timeout: Option<BigInt>,
}

/// PNG 过滤策略
#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum FilterStrategy {
    // 基本过滤器类型 (Basic RowFilter)
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

impl From<FilterStrategy> for export::IndexSet<export::FilterStrategy> {
    fn from(value: FilterStrategy) -> Self {
        let mut set = IndexSet::new();
        match value {
            FilterStrategy::None => {
                set.insert(export::FilterStrategy::Basic(export::RowFilter::None));
            }
            FilterStrategy::Sub => {
                set.insert(export::FilterStrategy::Basic(export::RowFilter::Sub));
            }
            FilterStrategy::Up => {
                set.insert(export::FilterStrategy::Basic(export::RowFilter::Up));
            }
            FilterStrategy::Average => {
                set.insert(export::FilterStrategy::Basic(export::RowFilter::Average));
            }
            FilterStrategy::Paeth => {
                set.insert(export::FilterStrategy::Basic(export::RowFilter::Paeth));
            }
            FilterStrategy::MinSum => {
                set.insert(export::FilterStrategy::MinSum);
            }
            FilterStrategy::Entropy => {
                set.insert(export::FilterStrategy::Entropy);
            }
            FilterStrategy::Bigrams => {
                set.insert(export::FilterStrategy::Bigrams);
            }
            FilterStrategy::BigEnt => {
                set.insert(export::FilterStrategy::BigEnt);
            }
            FilterStrategy::Brute => {
                // Brute 需要 num_lines 和 level 参数，使用默认值
                set.insert(export::FilterStrategy::Brute {
                    num_lines: 0,
                    level: 6,
                });
            }
        }
        set
    }
}

#[napi(discriminant = "type2")]
pub enum Deflater {
    /// 使用 libdeflater.
    Libdeflater {
        /// 对文件使用哪个压缩级别 （0-12）
        compression: u8,
    },
    // Zopfli 需要额外的配置，暂不支持
    // #[cfg(feature = "zopfli")]
    // /// 使用更好但速度较慢的 Zopfli 实现
    // Zopfli {
    //     // 要执行的压缩迭代次数。15 次迭代就可以了
    //     // 对于小文件，但较大的文件需要使用
    //     // 更少的迭代，否则它们会太慢。
    //     iterations: NonZeroU8,
    // },
}

impl From<Deflater> for export::Deflater {
    fn from(value: Deflater) -> Self {
        match value {
            Deflater::Libdeflater { compression } => export::Deflater::Libdeflater { compression },
        }
    }
}

#[napi(discriminant = "type2")]
pub enum StripChunks {
    /// 无
    None,
    /// 删除特定块, 长度为 4 的字符串数组，如 ["tEXt", "iTXt"]
    Strip(Array<'static>),
    /// 删除所有不会影响图像显示的数据块
    Safe,
    /// 删除除这些之外的所有非关键块, 长度为 4 的字符串数组
    Keep(Array<'static>),
    /// 所有非关键块
    All,
}

// ! 未写全: Strip 和 Keep 变体需要从 JS Array 解析 chunk 名称，当前实现忽略了 Array 参数
impl From<StripChunks> for export::StripChunks {
    fn from(value: StripChunks) -> Self {
        match value {
            StripChunks::None => export::StripChunks::None,
            StripChunks::Strip(_arr) => {
                // TODO: 从 Array 中解析 [u8; 4] chunk 名称
                // 每个元素应该是长度为 4 的字符串，如 "tEXt", "iTXt" 等
                export::StripChunks::Strip(IndexSet::new())
            }
            StripChunks::Safe => export::StripChunks::Safe,
            StripChunks::Keep(_arr) => {
                // TODO: 从 Array 中解析 [u8; 4] chunk 名称
                export::StripChunks::Keep(IndexSet::new())
            }
            StripChunks::All => export::StripChunks::All,
        }
    }
}

impl From<NapiOxiPngOptions> for export::OxiPngOptions {
    fn from(value: NapiOxiPngOptions) -> Self {
        let timeout = value.timeout.map(|t| {
            let (_signed, millis, _is_lossless) = t.get_u64();
            Duration::from_millis(millis)
        });

        export::OxiPngOptions {
            fix_errors: value.fix_errors,
            force: value.force,
            filters: value.filters.into(),
            interlace: value.interlace,
            optimize_alpha: value.optimize_alpha,
            bit_depth_reduction: value.bit_depth_reduction,
            color_type_reduction: value.color_type_reduction,
            palette_reduction: value.palette_reduction,
            grayscale_reduction: value.grayscale_reduction,
            idat_recoding: value.idat_recoding,
            scale_16: value.scale_16,
            strip: value.strip.into(),
            deflater: value.deflater.into(),
            fast_evaluation: value.fast_evaluation,
            timeout,
            ..Default::default()
        }
    }
}

impl From<Object<'_>> for NapiOxiPngOptions {
    fn from(value: Object) -> Self {
        Self {
            fix_errors: value.get_named_property::<bool>("fixErrors").unwrap_or(false),
            force: value.get_named_property::<bool>("force").unwrap_or(false),
            filters: value
                .get_named_property::<FilterStrategy>("filters")
                .unwrap_or(FilterStrategy::Entropy),
            interlace: value
                .get_named_property::<Option<bool>>("interlace")
                .unwrap_or(Some(false)),
            optimize_alpha: value.get_named_property::<bool>("optimizeAlpha").unwrap_or(false),
            bit_depth_reduction: value
                .get_named_property::<bool>("bitDepthReduction")
                .unwrap_or(true),
            color_type_reduction: value
                .get_named_property::<bool>("colorTypeReduction")
                .unwrap_or(true),
            palette_reduction: value
                .get_named_property::<bool>("paletteReduction")
                .unwrap_or(true),
            grayscale_reduction: value
                .get_named_property::<bool>("grayscaleReduction")
                .unwrap_or(true),
            idat_recoding: value.get_named_property::<bool>("idatRecoding").unwrap_or(true),
            scale_16: value.get_named_property::<bool>("scale16").unwrap_or(false),
            strip: value
                .get_named_property::<StripChunks>("strip")
                .unwrap_or(StripChunks::None),
            deflater: value
                .get_named_property::<Deflater>("deflater")
                .unwrap_or(Deflater::Libdeflater { compression: 12 }),
            fast_evaluation: value.get_named_property::<bool>("fastEvaluation").unwrap_or(true),
            timeout: value
                .get_named_property::<Option<BigInt>>("timeout")
                .unwrap_or(None),
        }
    }
}
