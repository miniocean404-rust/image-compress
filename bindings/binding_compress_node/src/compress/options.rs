// use std::time::Duration;
// use napi_derive::napi;
// use image_compress::export::*;
//
//
// #[napi(object)]
// pub struct OxiPngOptions {
//     /// Attempt to fix errors when decoding the input file rather than returning an `Err`.
//     ///
//     /// Default: `false`
//     pub fix_errors: bool,
//     /// Write to output even if there was no improvement in compression.
//     ///
//     /// Default: `false`
//     pub force: bool,
//     /// Which RowFilters to try on the file
//     ///
//     /// Default: `None,Sub,Entropy,Bigrams`
//     pub filter: IndexSet<RowFilter>,
//     /// Whether to change the interlacing type of the file.
//     ///
//     /// `None` will not change the current interlacing type.
//     ///
//     /// `Some(x)` will change the file to interlacing mode `x`.
//     ///
//     /// Default: `Some(Interlacing::None)`
//     pub interlace: Option<Interlacing>,
//     /// Whether to allow transparent pixels to be altered to improve compression.
//     pub optimize_alpha: bool,
//     /// Whether to attempt bit depth reduction
//     ///
//     /// Default: `true`
//     pub bit_depth_reduction: bool,
//     /// Whether to attempt color type reduction
//     ///
//     /// Default: `true`
//     pub color_type_reduction: bool,
//     /// Whether to attempt palette reduction
//     ///
//     /// Default: `true`
//     pub palette_reduction: bool,
//     /// Whether to attempt grayscale reduction
//     ///
//     /// Default: `true`
//     pub grayscale_reduction: bool,
//     /// Whether to perform recoding of IDAT and other compressed chunks
//     ///
//     /// If any type of reduction is performed, IDAT recoding will be performed
//     /// regardless of this setting
//     ///
//     /// Default: `true`
//     pub idat_recoding: bool,
//     /// Whether to forcibly reduce 16-bit to 8-bit by scaling
//     ///
//     /// Default: `false`
//     pub scale_16: bool,
//     /// Which chunks to strip from the PNG file, if any
//     ///
//     /// Default: `None`
//     pub strip: StripChunks,
//     /// Which DEFLATE algorithm to use
//     ///
//     /// Default: `Libdeflater`
//     pub deflate: Deflaters,
//     /// Whether to use fast evaluation to pick the best filter
//     ///
//     /// Default: `true`
//     pub fast_evaluation: bool,
//
//     /// Maximum amount of time to spend on optimizations.
//     /// Further potential optimizations are skipped if the timeout is exceeded.
//     pub timeout: Option<Duration>,
// }
//
// #[napi(object)]
// pub struct ImageQuantOptions {
//     // imagequant 默认最小值 0
//     pub min_quality: u8,
//
//     // imagequant 默认最大值 100, 如果 max 小于 100，库将尝试使用更少的颜色。 颜色较少的图像并不总是较小，因为它会导致抖动增加。
//     // 如果最小值为 0 最大值为 100 那么它会在 100 的基础上尽力压缩
//     pub max_quality: u8,
//
//     // 1 - 10
//     // 较快的速度会生成较低质量的图像，但对于实时生成图像可能很有用
//     // default: 4
//     pub speed: i32,
//
//     // 要忽略的最低有效 bit 的数目。用于生成 VGA, 15 位纹理或其他复古平台的调色板，越大压缩的越小
//     // 0 - 4
//     pub min_posterization: u8,
//
//     // 设置为 1.0 以获得平滑的图像，越小压缩越小
//     pub dithering: f32,
//
//     // 默认值是sRGB (~1/2.2)，，越小压缩越小，不知道做什么的
//     // 0 - 1
//     pub gamma: f64,
//
//     // 将透明颜色移动到调色板中的最后一个条目
//     // 这对于PNG来说效率较低，但某些有缺陷的软件却需要这样做
//     // true 会增大大小
//     pub last_index_transparent: bool,
// }
//
// #[napi(object)]
// pub struct MozJpegOptions {
//     /// 质量, 推荐 60-80. 范围：`1..=100`
//     pub quality: f32,
//
//     /// 设置图像的渐进模式
//     pub progressive: bool,
//
//     /// 设置为 false 可以毫无理由地使文件变大
//     pub optimize_coding: bool,
//
//     /// 非 0 （1..=100） 它将使用MozJPEG的平滑。
//     pub smoothing: u8,
//
//     /// 设置正在写入的 JPEG 的颜色空间，不同于输入的颜色空间
//     pub color_space: MozJpegColorSpace,
//
//     /// 指定在网格量化期间是否应考虑多次扫描。
//     pub trellis_multipass: bool,
//
//     /// 设置色度子采样，保留为“None”以使用自动子采样
//     pub chroma_subsample: Option<u8>,
//
//     /// 是否使用特定的量化表。替代质量（quality）设置。
//     pub luma: bool,
//
//     /// 是否使用特定的量化表的颜色。替代质量（quality）设置。
//     pub chroma: bool,
//
//     pub qtable: Option<QtableOptimize>,
// }
//
// // impl From<Object> for MozJpegOptions {
// //     fn from(value: Object) -> Self {
// //         MozJpegOptions {
// //             quality: value.get("quality").unwrap().unwrap_or(0),
// //         }
// //     }
// // }
//
// #[napi(object)]
// pub struct WebPOptions {
//     pub lossless: i32,
//     pub quality: f32,
//     pub method: i32,
//     pub image_hint: WebPImageHint,
//     pub target_size: i32,
//     pub target_PSNR: f32,
//     pub segments: i32,
//     pub sns_strength: i32,
//     pub filter_strength: i32,
//     pub filter_sharpness: i32,
//     pub filter_type: i32,
//     pub autofilter: i32,
//     pub alpha_compression: i32,
//     pub alpha_filtering: i32,
//     pub alpha_quality: i32,
//     pub pass: i32,
//     pub show_compressed: i32,
//     pub preprocessing: i32,
//     pub partitions: i32,
//     pub partition_limit: i32,
//     pub emulate_jpeg_size: i32,
//     pub thread_level: i32,
//     pub low_memory: i32,
//     pub near_lossless: i32,
//     pub exact: i32,
//     pub use_delta_palette: i32,
//     pub use_sharp_yuv: i32,
//     pub qmin: i32,
//     pub qmax: i32,
// }
//
// #[napi(object)]
// pub struct AvifOptions {
//     /// 质量 `1..=100`
//     pub quality: f32,
//
//     /// Alpha 通道的独立质量 `1..=100`
//     pub alpha_quality: Option<f32>,
//
//     /// 压缩速度 (effort) `1..=10`
//     ///
//     /// 1 = 非常非常慢，但是最大的压缩
//     /// 10 = 快速，但文件大小较大，质量较差。
//     pub speed: u8,
//
//     /// 更改图像中颜色通道的存储方式。
//     ///
//     /// 请注意，这只是AVIF文件的内部细节，不会改变编码函数输入的颜色空间。
//     pub color_space: AvifColorSpace,
//
//     /// 配置透明图像中颜色通道的处理
//     pub alpha_color_mode: AlphaColorMode,
// }
