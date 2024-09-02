use image_compress::export;
use napi_derive::napi;

#[napi(object)]
pub struct MozJpegOptions {
    /// 质量, 推荐 60-80. 范围：`1..=100`
    pub quality: f64,

    /// 设置图像的渐进模式
    pub progressive: bool,

    /// 设置为 false 可以毫无理由地使文件变大
    pub optimize_coding: bool,

    /// 非 0 （1..=100） 它将使用MozJPEG的平滑。
    pub smoothing: u8,

    /// 设置正在写入的 JPEG 的颜色空间，不同于输入的颜色空间
    pub color_space: ColorSpace,

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

#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum ColorSpace {
    /// error/unspecified
    JCS_UNKNOWN,
    /// monochrome
    JCS_GRAYSCALE,
    /// red/green/blue as specified by the `RGB_RED`, `RGB_GREEN`, `RGB_BLUE`, and `RGB_PIXELSIZE` macros
    JCS_RGB,
    /// Y/Cb/Cr (also known as YUV)
    JCS_YCbCr,
    /// C/M/Y/K
    JCS_CMYK,
    /// Y/Cb/Cr/K
    JCS_YCCK,
    /// red/green/blue
    JCS_EXT_RGB,
    /// red/green/blue/x
    /// When `out_color_space` it set to `JCS_EXT_RGBX`, `JCS_EXT_BGRX`, `JCS_EXT_XBGR`,
    /// or `JCS_EXT_XRGB` during decompression, the X byte is undefined, and in
    /// order to ensure the best performance, libjpeg-turbo can set that byte to
    /// whatever value it wishes.
    JCS_EXT_RGBX,
    /// blue/green/red
    JCS_EXT_BGR,
    /// blue/green/red/x
    JCS_EXT_BGRX,
    /// x/blue/green/red
    JCS_EXT_XBGR,
    /// x/red/green/blue
    JCS_EXT_XRGB,
    /// Use the following colorspace constants to
    /// ensure that the X byte is set to 0xFF, so that it can be interpreted as an
    /// opaque alpha channel.
    ///
    /// red/green/blue/alpha
    JCS_EXT_RGBA,
    /// blue/green/red/alpha
    JCS_EXT_BGRA,
    /// alpha/blue/green/red
    JCS_EXT_ABGR,
    /// alpha/red/green/blue
    JCS_EXT_ARGB,
    /// 5-bit red/6-bit green/5-bit blue
    JCS_RGB565,
}

#[allow(non_camel_case_types)]
#[napi(string_enum)]
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

impl From<MozJpegOptions> for export::MozJpegOptions {
    fn from(value: MozJpegOptions) -> Self {
        export::MozJpegOptions {
            quality: value.quality as f32,
            progressive: value.progressive,
            optimize_coding: value.optimize_coding,
            smoothing: value.smoothing,
            color_space: value.color_space.into(),
            trellis_multipass: value.trellis_multipass,
            chroma_subsample: value.chroma_subsample,
            luma: value.luma,
            chroma: value.chroma,
            qtable: value.qtable,
        }
    }
}
