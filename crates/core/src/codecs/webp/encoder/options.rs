use libwebp_sys::WebPImageHint;
use crate::codecs::OptionsTrait;

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct WebPOptions {
    pub lossless: i32,
    pub quality: f32,
    pub method: i32,
    pub image_hint: WebPImageHint,
    pub target_size: i32,
    pub target_PSNR: f32,
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

impl OptionsTrait for WebPOptions {}

impl Default for WebPOptions {
    fn default() -> Self {
        Self {
            lossless: 0,
            quality: 75.0,
            method: 4,
            image_hint: WebPImageHint::WEBP_HINT_DEFAULT,
            target_size: 0,
            target_PSNR: 0.0,
            segments: 4,
            sns_strength: 50,
            filter_strength: 60,
            filter_sharpness: 0,
            filter_type: 1,
            autofilter: 0,
            alpha_compression: 1,
            alpha_filtering: 1,
            alpha_quality: 100,
            pass: 1,
            show_compressed: 0,
            preprocessing: 0,
            partitions: 0,
            partition_limit: 0,
            emulate_jpeg_size: 0,
            thread_level: 0,
            low_memory: 0,
            near_lossless: 100,
            exact: 0,
            use_delta_palette: 0,
            use_sharp_yuv: 0,
            qmin: 0,
            qmax: 100,
        }
    }
}

impl From<WebPOptions> for webp::WebPConfig {
    fn from(value: WebPOptions) -> Self {
        let mut config = webp::WebPConfig::new().unwrap();

        config.lossless = value.lossless;
        config.quality = value.quality;
        config.method = value.method;
        config.image_hint = value.image_hint;
        config.target_size = value.target_size;
        config.target_PSNR = value.target_PSNR;
        config.segments = value.segments;
        config.sns_strength = value.sns_strength;
        config.filter_strength = value.filter_strength;
        config.filter_sharpness = value.filter_sharpness;
        config.filter_type = value.filter_type;
        config.autofilter = value.autofilter;
        config.alpha_compression = value.alpha_compression;
        config.alpha_filtering = value.alpha_filtering;
        config.alpha_quality = value.alpha_quality;
        config.pass = value.pass;
        config.show_compressed = value.show_compressed;
        config.preprocessing = value.preprocessing;
        config.partitions = value.partitions;
        config.partition_limit = value.partition_limit;
        config.emulate_jpeg_size = value.emulate_jpeg_size;
        config.thread_level = value.thread_level;
        config.low_memory = value.low_memory;
        config.near_lossless = value.near_lossless;
        config.exact = value.exact;
        config.use_delta_palette = value.use_delta_palette;
        config.use_sharp_yuv = value.use_sharp_yuv;
        config.qmin = value.qmin;
        config.qmax = value.qmax;

        config
    }
}