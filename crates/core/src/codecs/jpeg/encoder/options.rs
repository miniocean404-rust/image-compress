pub type MozJpegColorSpace = mozjpeg::ColorSpace;

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
    pub color_space: MozJpegColorSpace,

    /// 指定在网格量化期间是否应考虑多次扫描。(会导致偏大一点点)
    pub trellis_multipass: bool,

    /// 优化渐进式扫描顺序，使渐进式图像文件更小
    pub optimize_scans: bool,

    /// 设置色度子采样，保留为"None"以使用自动子采样（根据质量自动选择）
    pub chroma_subsample: Option<u8>,

    /// 是否自动根据质量选择色度子采样
    /// 高质量(>90)用4:4:4，中等质量(>70)用4:2:2，低质量用 4:2:0
    pub auto_chroma_subsample: bool,

    /// 是否使用特定的量化表。替代质量（quality）设置。
    pub luma: bool,

    /// 是否使用特定的量化表的颜色。替代质量（quality）设置。
    pub chroma: bool,

    /// 亮度量化表
    pub qtable: Option<QtableOptimize>,

    /// 色度专用量化表（如果为 None 则使用 qtable）
    pub qtable_chroma: Option<QtableOptimizeChroma>,

    /// 是否自动根据质量选择优化量化表
    /// 高质量(>=80)时自动使用 MSSSIM 量化表以获得更好的视觉质量
    pub auto_qtable: bool,
}

impl Default for MozJpegOptions {
    fn default() -> Self {
        Self {
            quality: 70.,
            progressive: true,
            optimize_coding: true,
            smoothing: 0,
            color_space: mozjpeg::ColorSpace::JCS_YCbCr,
            // 启用 Trellis 多遍优化，MozJpeg 核心优势，可减少 3-5% 文件大小
            trellis_multipass: true,
            // 启用渐进式扫描优化，使渐进式图像文件更小
            optimize_scans: true,
            chroma_subsample: None,
            // 默认启用自动色度子采样
            auto_chroma_subsample: true,
            luma: false,
            chroma: false,
            qtable: None,
            qtable_chroma: None,
            // 默认启用自动量化表选择，高质量时使用 MSSSIM 优化
            auto_qtable: true,
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

/// 色度专用量化表
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum QtableOptimizeChroma {
    /// Annex K 色度量化表
    AnnexK_Chroma,
    /// MS-SSIM 优化的色度量化表
    MSSSIM_Chroma,
    /// PSNR-HVS 优化的色度量化表
    PSNRHVS_Chroma,
}

// Jxl encoder
pub type JxlOptions = zune_core::options::EncoderOptions;
