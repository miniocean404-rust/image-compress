use napi::bindgen_prelude::*;
use napi_derive::napi;

use image_compress::compress::index::ImageCompression;
use image_compress::utils::mime::SupportedFileTypes;

#[napi(object)]
pub struct CompressInfo {
    pub name: String,

    pub path: String,

    pub file_type: Kind,

    pub quality: i8,

    pub before_size: BigInt,

    pub after_size: BigInt,

    pub rate: f64,

    pub mem: Buffer,
}

// string_enum 转化为字符串枚举
#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum Kind {
    jpeg,
    png,
    gif,
    webp,
    unknown,
}

impl From<SupportedFileTypes> for Kind {
    fn from(s: SupportedFileTypes) -> Self {
        match s {
            SupportedFileTypes::Jpeg => Kind::jpeg,
            SupportedFileTypes::Png => Kind::png,
            SupportedFileTypes::Gif => Kind::gif,
            SupportedFileTypes::WebP => Kind::webp,
            SupportedFileTypes::Unknown => Kind::unknown,
        }
    }
}

#[napi(js_name = "getImageInfo")]
#[tracing::instrument(level = "info", skip_all)]
async fn get_image_info(file: String, quality: i8) -> Result<CompressInfo> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    let info = start_compress(file, quality).await?;

    let ImageCompression {
        name,
        path,
        file_type,
        quality,
        before_size,
        after_size,
        rate,
        mem,
        ..
    } = info;

    let info = CompressInfo {
        name,
        path,
        file_type: Kind::from(file_type),
        quality,
        before_size: BigInt::from(before_size),
        after_size: BigInt::from(after_size),
        rate,
        mem: Buffer::from(mem),
    };

    Ok(info)
    // Ok(info)
}

#[allow(dead_code)]
async fn start_compress(file: String, quality: i8) -> Result<ImageCompression> {
    let mut info = ImageCompression::new(file, quality);

    info.start_mem_compress(false)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, format!("compress 失败:, {}", e)))?;

    Ok(info)
}
