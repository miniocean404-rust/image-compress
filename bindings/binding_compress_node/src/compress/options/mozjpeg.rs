use image_compress::export;
use napi::bindgen_prelude::Object;
use napi_derive::napi;

#[napi(object, js_name = "MozJpegOptions")]
pub struct NapiMozJpegOptions {
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

impl From<Object> for NapiMozJpegOptions {
    fn from(value: Object) -> Self {
        Self {
            quality: value.get_named_property::<f64>("quality").unwrap(),
            progressive: value.get_named_property::<bool>("progressive").unwrap(),
            optimize_coding: value.get_named_property::<bool>("optimizeCoding").unwrap(),
            smoothing: value.get_named_property::<u8>("smoothing").unwrap(),
            color_space: value
                .get_named_property::<ColorSpace>("colorSpace")
                .unwrap(),
            trellis_multipass: value
                .get_named_property::<bool>("trellisMultipass")
                .unwrap(),
            chroma_subsample: value
                .get_named_property::<Option<u8>>("chromaSubsample")
                .unwrap(),
            luma: value.get_named_property::<bool>("luma").unwrap(),
            chroma: value.get_named_property::<bool>("chroma").unwrap(),
            qtable: value
                .get_named_property::<Option<QtableOptimize>>("qtable")
                .unwrap(),
        }
    }
}

impl From<NapiMozJpegOptions> for export::MozJpegOptions {
    fn from(value: NapiMozJpegOptions) -> Self {
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
            qtable: value.qtable.map(|e| e.into()),
        }
    }
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

impl From<ColorSpace> for export::MozJpegColorSpace {
    fn from(value: ColorSpace) -> Self {
        match value {
            ColorSpace::JCS_UNKNOWN => export::MozJpegColorSpace::JCS_UNKNOWN,
            ColorSpace::JCS_GRAYSCALE => export::MozJpegColorSpace::JCS_GRAYSCALE,
            ColorSpace::JCS_RGB => export::MozJpegColorSpace::JCS_RGB,
            ColorSpace::JCS_YCbCr => export::MozJpegColorSpace::JCS_YCbCr,
            ColorSpace::JCS_CMYK => export::MozJpegColorSpace::JCS_CMYK,
            ColorSpace::JCS_YCCK => export::MozJpegColorSpace::JCS_YCCK,
            ColorSpace::JCS_EXT_RGB => export::MozJpegColorSpace::JCS_EXT_RGB,
            ColorSpace::JCS_EXT_RGBX => export::MozJpegColorSpace::JCS_EXT_RGBX,
            ColorSpace::JCS_EXT_BGR => export::MozJpegColorSpace::JCS_EXT_BGR,
            ColorSpace::JCS_EXT_BGRX => export::MozJpegColorSpace::JCS_EXT_BGRX,
            ColorSpace::JCS_EXT_XBGR => export::MozJpegColorSpace::JCS_EXT_XBGR,
            ColorSpace::JCS_EXT_XRGB => export::MozJpegColorSpace::JCS_EXT_XRGB,
            ColorSpace::JCS_EXT_RGBA => export::MozJpegColorSpace::JCS_EXT_RGBA,
            ColorSpace::JCS_EXT_BGRA => export::MozJpegColorSpace::JCS_EXT_BGRA,
            ColorSpace::JCS_EXT_ABGR => export::MozJpegColorSpace::JCS_EXT_ABGR,
            ColorSpace::JCS_EXT_ARGB => export::MozJpegColorSpace::JCS_EXT_ARGB,
            ColorSpace::JCS_RGB565 => export::MozJpegColorSpace::JCS_RGB565,
        }
    }
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

impl From<QtableOptimize> for export::QtableOptimize {
    fn from(value: QtableOptimize) -> Self {
        match value {
            QtableOptimize::AhumadaWatsonPeterson => export::QtableOptimize::AhumadaWatsonPeterson,
            QtableOptimize::AnnexK_Luma => export::QtableOptimize::AnnexK_Luma,
            QtableOptimize::Flat => export::QtableOptimize::Flat,
            QtableOptimize::KleinSilversteinCarney => {
                export::QtableOptimize::KleinSilversteinCarney
            }
            QtableOptimize::MSSSIM_Luma => export::QtableOptimize::MSSSIM_Luma,
            QtableOptimize::NRobidoux => export::QtableOptimize::NRobidoux,
            QtableOptimize::PSNRHVS_Luma => export::QtableOptimize::PSNRHVS_Luma,
            QtableOptimize::PetersonAhumadaWatson => export::QtableOptimize::PetersonAhumadaWatson,
            QtableOptimize::WatsonTaylorBorthwick => export::QtableOptimize::WatsonTaylorBorthwick,
        }
    }
}
