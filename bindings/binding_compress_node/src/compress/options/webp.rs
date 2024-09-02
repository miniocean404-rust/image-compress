use image_compress::export;
use napi::bindgen_prelude::Object;
use napi_derive::napi;

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

impl From<Object> for WebPOptions {
    fn from(value: Object) -> Self {
        Self {
            lossless: value.get_named_property::<i32>("lossless").unwrap(),
            quality: value.get_named_property::<f64>("quality").unwrap(),
            method: value.get_named_property::<i32>("method").unwrap(),
            image_hint: value
                .get_named_property::<WebPImageHint>("imageHint")
                .unwrap(),
            target_size: value.get_named_property::<i32>("targetSize").unwrap(),
            target_psnr: value.get_named_property::<f64>("targetPSNR").unwrap(),
            segments: value.get_named_property::<i32>("segments").unwrap(),
            sns_strength: value.get_named_property::<i32>("snsStrength").unwrap(),
            filter_strength: value.get_named_property::<i32>("filterStrength").unwrap(),
            filter_sharpness: value.get_named_property::<i32>("filterSharpness").unwrap(),
            filter_type: value.get_named_property::<i32>("filterType").unwrap(),
            autofilter: value.get_named_property::<i32>("autofilter").unwrap(),
            alpha_compression: value.get_named_property::<i32>("alphaCompression").unwrap(),
            alpha_filtering: value.get_named_property::<i32>("alphaFiltering").unwrap(),
            alpha_quality: value.get_named_property::<i32>("alphaQuality").unwrap(),
            pass: value.get_named_property::<i32>("pass").unwrap(),
            show_compressed: value.get_named_property::<i32>("showCompressed").unwrap(),
            preprocessing: value.get_named_property::<i32>("preprocessing").unwrap(),
            partitions: value.get_named_property::<i32>("partitions").unwrap(),
            partition_limit: value.get_named_property::<i32>("partitionLimit").unwrap(),
            emulate_jpeg_size: value.get_named_property::<i32>("emulateJpegSize").unwrap(),
            thread_level: value.get_named_property::<i32>("threadLevel").unwrap(),
            low_memory: value.get_named_property::<i32>("lowMemory").unwrap(),
            near_lossless: value.get_named_property::<i32>("nearLossless").unwrap(),
            exact: value.get_named_property::<i32>("exact").unwrap(),
            use_delta_palette: value.get_named_property::<i32>("useDeltaPalette").unwrap(),
            use_sharp_yuv: value.get_named_property::<i32>("useSharpYUV").unwrap(),
            qmin: value.get_named_property::<i32>("qmin").unwrap(),
            qmax: value.get_named_property::<i32>("qmax").unwrap(),
        }
    }
}

impl From<WebPOptions> for export::WebPOptions {
    fn from(value: WebPOptions) -> Self {
        export::WebPOptions {
            lossless: value.lossless,
            quality: value.quality as f32,
            method: value.method,
            image_hint: value.image_hint.into(),
            target_size: value.target_size,
            target_PSNR: value.target_psnr as f32,
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

#[allow(non_camel_case_types)]
#[napi]
pub enum WebPImageHint {
    WEBP_HINT_DEFAULT = 0,
    WEBP_HINT_PICTURE = 1,
    WEBP_HINT_PHOTO = 2,
    WEBP_HINT_GRAPH = 3,
    WEBP_HINT_LAST = 4,
}

impl From<WebPImageHint> for export::WebPImageHint {
    fn from(value: WebPImageHint) -> Self {
        match value {
            WebPImageHint::WEBP_HINT_DEFAULT => export::WebPImageHint::WEBP_HINT_DEFAULT,
            WebPImageHint::WEBP_HINT_PICTURE => export::WebPImageHint::WEBP_HINT_PICTURE,
            WebPImageHint::WEBP_HINT_PHOTO => export::WebPImageHint::WEBP_HINT_PHOTO,
            WebPImageHint::WEBP_HINT_GRAPH => export::WebPImageHint::WEBP_HINT_GRAPH,
            WebPImageHint::WEBP_HINT_LAST => export::WebPImageHint::WEBP_HINT_LAST,
        }
    }
}
