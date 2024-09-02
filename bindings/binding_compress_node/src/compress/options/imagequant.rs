use image_compress::export;
use napi::bindgen_prelude::Object;
use napi_derive::napi;

#[napi(object)]
pub struct ImageQuantOptions {
    // imagequant 默认最小值 0
    pub min_quality: u8,

    // imagequant 默认最大值 100, 如果 max 小于 100，库将尝试使用更少的颜色。 颜色较少的图像并不总是较小，因为它会导致抖动增加。
    // 如果最小值为 0 最大值为 100 那么它会在 100 的基础上尽力压缩
    pub max_quality: u8,

    // 1 - 10
    // 较快的速度会生成较低质量的图像，但对于实时生成图像可能很有用
    // default: 4
    pub speed: i32,

    // 要忽略的最低有效 bit 的数目。用于生成 VGA, 15 位纹理或其他复古平台的调色板，越大压缩的越小
    // 0 - 4
    pub min_posterization: u8,

    // 设置为 1.0 以获得平滑的图像，越小压缩越小
    pub dithering: f64,

    // 默认值是sRGB (~1/2.2)，，越小压缩越小，不知道做什么的
    // 0 - 1
    pub gamma: f64,

    // 将透明颜色移动到调色板中的最后一个条目
    // 这对于PNG来说效率较低，但某些有缺陷的软件却需要这样做
    // true 会增大大小
    pub last_index_transparent: bool,
}

impl From<ImageQuantOptions> for export::ImageQuantOptions {
    fn from(value: ImageQuantOptions) -> Self {
        export::ImageQuantOptions {
            min_quality: value.min_quality,
            max_quality: value.max_quality,
            speed: value.speed,
            min_posterization: value.min_posterization,
            dithering: value.dithering as f32,
            gamma: value.gamma,
            last_index_transparent: value.last_index_transparent,
        }
    }
}

impl From<Object> for ImageQuantOptions {
    fn from(value: Object) -> Self {
        Self {
            min_quality: value.get_named_property::<u8>("minQuality").unwrap(),
            max_quality: value.get_named_property::<u8>("maxQuality").unwrap(),
            speed: value.get_named_property::<i32>("speed").unwrap(),
            min_posterization: value.get_named_property::<u8>("minPosterization").unwrap(),
            dithering: value.get_named_property::<f64>("dithering").unwrap(),
            gamma: value.get_named_property::<f64>("gamma").unwrap(),
            last_index_transparent: value
                .get_named_property::<bool>("lastIndexTransparent")
                .unwrap(),
        }
    }
}
