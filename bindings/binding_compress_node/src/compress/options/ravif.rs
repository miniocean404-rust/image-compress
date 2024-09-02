use napi_derive::napi;
use image_compress::export;

#[napi(object)]
pub struct AvifOptions {
    /// 质量 `1..=100`
    pub quality: f64,

    /// Alpha 通道的独立质量 `1..=100`
    pub alpha_quality: Option<f64>,

    /// 压缩速度 (effort) `1..=10`
    ///
    /// 1 = 非常非常慢，但是最大的压缩
    /// 10 = 快速，但文件大小较大，质量较差。
    pub speed: u8,

    /// 更改图像中颜色通道的存储方式。
    ///
    /// 请注意，这只是AVIF文件的内部细节，不会改变编码函数输入的颜色空间。
    pub color_space: ColorSpace,

    /// 配置透明图像中颜色通道的处理
    pub alpha_color_mode: AlphaColorMode,
}

#[napi(string_enum)]
pub enum ColorSpace {
    /// Standard color space for photographic content. Usually the best choice.
    /// This library always uses full-resolution color (4:4:4).
    /// This library will automatically choose between BT.601 or BT.709.
    YCbCr,
    /// RGB channels are encoded without colorspace transformation.
    /// Usually results in larger file sizes, and is less compatible than `YCbCr`.
    /// Use only if the content really makes use of RGB, e.g. anaglyph images or RGB subpixel anti-aliasing.
    RGB,
}

#[napi(string_enum)]
pub enum AlphaColorMode {
    /// Use unassociated alpha channel and leave color channels unchanged, even if there's redundant color data in transparent areas.
    UnassociatedDirty,
    /// Use unassociated alpha channel, but set color channels of transparent areas to a solid color to eliminate invisible data and improve compression.
    UnassociatedClean,
    /// Store color channels of transparent images in premultiplied form.
    /// This requires support for premultiplied alpha in AVIF decoders.
    ///
    /// It may reduce file sizes due to clearing of fully-transparent pixels, but
    /// may also increase file sizes due to creation of new edges in the color channels.
    ///
    /// Note that this is only internal detail for the AVIF file.
    /// It does not change meaning of `RGBA` in this library — it's always unassociated.
    Premultiplied,
}


impl From<AvifOptions> for export::AvifOptions {
    fn from(value: AvifOptions) -> Self {
        export::AvifOptions {
            quality: value.quality.into(),
            alpha_quality: value.alpha_quality.into(),
            speed: value.speed,
            color_space: match value.color_space {
                ColorSpace::YCbCr => export::AvifColorSpace::YCbCr,
                ColorSpace::RGB => export::AvifColorSpace::RGB,
            },
            alpha_color_mode: match value.alpha_color_mode {
                AlphaColorMode::UnassociatedDirty => export::AlphaColorMode::UnassociatedDirty,
                AlphaColorMode::UnassociatedClean => export::AlphaColorMode::UnassociatedClean,
                AlphaColorMode::Premultiplied => export::AlphaColorMode::Premultiplied,
            },
        }
    }
}