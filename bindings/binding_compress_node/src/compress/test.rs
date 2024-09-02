use image_compress::{
    compress::OptionsTrait,
    export::{ImageQuantOptions, MozJpegColorSpace, MozJpegOptions},
};

// use std::{any::Any, fmt::Debug};

// use image_compress::export::MozJpegColorSpace;

// #[derive(Debug)]
// pub struct MozJpegOptions {
//     /// 质量, 推荐 60-80. 范围：`1..=100`
//     pub quality: f32,

//     /// 设置图像的渐进模式
//     pub progressive: bool,

//     /// 设置为 false 可以毫无理由地使文件变大
//     pub optimize_coding: bool,

//     /// 非 0 （1..=100） 它将使用MozJPEG的平滑。
//     pub smoothing: u8,

//     /// 设置正在写入的 JPEG 的颜色空间，不同于输入的颜色空间
//     pub color_space: MozJpegColorSpace,

//     /// 指定在网格量化期间是否应考虑多次扫描。
//     pub trellis_multipass: bool,

//     /// 设置色度子采样，保留为“None”以使用自动子采样
//     pub chroma_subsample: Option<u8>,

//     /// 是否使用特定的量化表。替代质量（quality）设置。
//     pub luma: bool,

//     /// 是否使用特定的量化表的颜色。替代质量（quality）设置。
//     pub chroma: bool,

//     pub qtable: Option<QtableOptimize>,
// }

// impl Default for MozJpegOptions {
//     fn default() -> Self {
//         Self {
//             quality: 75.,
//             progressive: true,
//             optimize_coding: true,
//             smoothing: 0,
//             color_space: MozJpegColorSpace::JCS_YCbCr,
//             trellis_multipass: false,
//             chroma_subsample: None,
//             luma: false,
//             chroma: false,
//             qtable: None,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub enum QtableOptimize {
//     AhumadaWatsonPeterson,
//     AnnexK_Luma,
//     Flat,
//     KleinSilversteinCarney,
//     MSSSIM_Luma,
//     NRobidoux,
//     PSNRHVS_Luma,
//     PetersonAhumadaWatson,
//     WatsonTaylorBorthwick,
// }

// pub trait OptionsTrait: Any + Debug {
//     fn as_any(&self) -> &dyn Any;
// }

// impl OptionsTrait for MozJpegOptions {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub struct ImageQuantOptions {
//     // imagequant 默认最小值 0
//     pub min_quality: u8,

//     // imagequant 默认最大值 100, 如果 max 小于 100，库将尝试使用更少的颜色。 颜色较少的图像并不总是较小，因为它会导致抖动增加。
//     // 如果最小值为 0 最大值为 100 那么它会在 100 的基础上尽力压缩
//     pub max_quality: u8,

//     // 1 - 10
//     // 较快的速度会生成较低质量的图像，但对于实时生成图像可能很有用
//     // default: 4
//     pub speed: i32,

//     // 要忽略的最低有效 bit 的数目。用于生成 VGA, 15 位纹理或其他复古平台的调色板，越大压缩的越小
//     // 0 - 4
//     pub min_posterization: u8,

//     // 设置为 1.0 以获得平滑的图像，越小压缩越小
//     pub dithering: f32,

//     // 默认值是sRGB (~1/2.2)，，越小压缩越小，不知道做什么的
//     // 0 - 1
//     pub gamma: f64,

//     // 将透明颜色移动到调色板中的最后一个条目
//     // 这对于PNG来说效率较低，但某些有缺陷的软件却需要这样做
//     // true 会增大大小
//     pub last_index_transparent: bool,
// }

// impl OptionsTrait for ImageQuantOptions {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Default for ImageQuantOptions {
//     fn default() -> Self {
//         ImageQuantOptions {
//             min_quality: 0,
//             max_quality: 100,
//             speed: 1,
//             min_posterization: 4,
//             dithering: 0.0,
//             gamma: 0.1,
//             last_index_transparent: false,
//         }
//     }
// }

fn main() {
    let mozjpeg_options: Box<dyn OptionsTrait> = Box::new(ImageQuantOptions::default());
    let mozjpeg_options: Box<dyn OptionsTrait> = Box::new(MozJpegOptions::default());
}
