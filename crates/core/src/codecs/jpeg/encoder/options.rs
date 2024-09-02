use crate::codecs::OptionsTrait;

#[derive(Debug, Clone, Copy)]
/// Advanced options for MozJpeg encoding
pub struct MozJpegOptions {
    /// 质量, 推荐 60-80. 范围：`1..=100`
    pub quality: f32,

    /// 设置图像的渐进模式
    pub progressive: bool,

    /// 设置为 false 可以毫无理由地使文件变大
    pub optimize_coding: bool,

    /// 非 0 （1..=100） 它将使用MozJPEG的平滑。
    pub smoothing: u8,

    /// 设置正在写入的 JPEG 的颜色空间，不同于输入的颜色空间
    pub color_space: mozjpeg::ColorSpace,

    /// 指定在网格量化期间是否应考虑多次扫描。
    pub trellis_multipass: bool,

    /// 设置色度子采样，保留为“None”以使用自动子采样
    pub chroma_subsample: Option<u8>,

    /// 是否使用特定的量化表。替代质量（quality）设置。
    pub luma: bool,

    /// 是否使用特定的量化表的颜色。替代质量（quality）设置。
    pub chroma: bool,

    pub qtable: Option<QtableOptimize>,
}

impl OptionsTrait for MozJpegOptions {}

impl Default for MozJpegOptions {
    fn default() -> Self {
        Self {
            quality: 75.,
            progressive: true,
            optimize_coding: true,
            smoothing: 0,
            color_space: mozjpeg::ColorSpace::JCS_YCbCr,
            trellis_multipass: false,
            chroma_subsample: None,
            luma: false,
            chroma: false,
            qtable: None,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum QtableOptimize {
    AhumadaWatsonPeterson,
    AnnexK_Luma,
    Flat,
    KleinSilversteinCarney,
    MSSSIM_Luma,
    NRobidoux,
    PSNRHVS_Luma,
    PetersonAhumadaWatson,
    WatsonTaylorBorthwick,
}