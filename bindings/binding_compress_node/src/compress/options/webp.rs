use napi_derive::napi;
use image_compress::export;

#[allow(non_snake_case)]
#[napi(object)]
pub struct WebPOptions {
    pub lossless: i32,
    pub quality: f64,
    pub method: i32,
    pub image_hint: WebPImageHint,
    pub target_size: i32,
    pub target_psnr: f64,
    pub segments: i32,
    pub sns_strength: i32,
    pub filter_strength: i32,
    pub filter_sharpness: i32,
    pub filter_type: i32,
    pub autofilter: i32,
    pub alpha_compression: i32,
    pub alpha_filtering: i32,
    pub alpha_quality: i32,
    pub pass: i32,
    pub show_compressed: i32,
    pub preprocessing: i32,
    pub partitions: i32,
    pub partition_limit: i32,
    pub emulate_jpeg_size: i32,
    pub thread_level: i32,
    pub low_memory: i32,
    pub near_lossless: i32,
    pub exact: i32,
    pub use_delta_palette: i32,
    pub use_sharp_yuv: i32,
    pub qmin: i32,
    pub qmax: i32,
}

#[allow(non_camel_case_types)]
#[napi]
pub enum WebPImageHint {
    WEBP_HINT_DEFAULT = 0,
    WEBP_HINT_PICTURE = 1,
    WEBP_HINT_PHOTO = 2,
    WEBP_HINT_GRAPH = 3,
    WEBP_HINT_LAST = 4,
}

impl From<WebPOptions> for export::WebPOptions {
    fn from(value: WebPOptions) -> Self {
        export::WebPOptions {
            lossless: value.lossless,
            quality: value.quality.into(),
            method: value.method,
            image_hint: value.image_hint.into(),
            target_size: value.target_size,
            target_PSNR: value.target_psnr.into(),
            segments: value.segments,
            sns_strength: value.sns_strength,
            filter_strength: value.filter_strength,
            filter_sharpness: value.filter_sharpness,
            filter_type: value.filter_type,
            autofilter: value.autofilter,
            alpha_compression: value.alpha_compression,
            alpha_filtering: value.alpha_filtering,
            alpha_quality: value.alpha_quality,
            pass: value.pass,
            show_compressed: value.show_compressed,
            preprocessing: value.preprocessing,
            partitions: value.partitions,
            partition_limit: value.partition_limit,
            emulate_jpeg_size: value.emulate_jpeg_size,
            thread_level: value.thread_level,
            low_memory: value.low_memory,
            near_lossless: value.near_lossless,
            exact: value.exact,
            use_delta_palette: value.use_delta_palette,
            use_sharp_yuv: value.use_sharp_yuv,
            qmin: value.qmin,
            qmax: value.qmax,
        }
    }
}