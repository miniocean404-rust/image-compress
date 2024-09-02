use std::fs;
use image_compress::export::*;

use image_compress::compress::{ImageCompress};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use utils::file::mime::get_mime_for_memory;

use super::kind::Kind;

#[napi(object)]
pub struct CompressInfo {
    pub ext: Kind,

    pub before_size: BigInt,

    pub after_size: BigInt,

    pub rate: f64,

    pub compressed_image: Buffer,
}

#[napi(js_name = "compress")]
#[tracing::instrument(level = "info", skip_all)]
fn compress(
    file: String,
    // #[napi(
    //     ts_arg_type = "MozJpegOptions | OxiPngOptions | ImageQuantOptions | WebPOptions | AvifOptions"
    // )]
    options: Object,
) -> Result<CompressInfo> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    // let options: super::options::MozJpegOptions = options.into();
    let buffer = fs::read(file)?;
    let ext = get_mime_for_memory(&buffer);

    let mut info = ImageCompress::new(buffer).with_options(MozJpegOptions::default());

    // let option = match info.ext {
    //     image_compress::support::SupportedFileTypes::Jpeg => MozJpegOptions::default(),
    //     image_compress::support::SupportedFileTypes::Png => OxiPngOptions::max_compression(),
    //     image_compress::support::SupportedFileTypes::WebP => WebPOptions::default(),
    //     image_compress::support::SupportedFileTypes::Avif => AvifOptions::default(),
    //     image_compress::support::SupportedFileTypes::Unknown => {
    //         return Err(Error::new(
    //             Status::GenericFailure,
    //             "不支持的图片类型".to_string(),
    //         ))
    //     }
    // };

    info.compress()
        .map_err(|e| Error::new(Status::GenericFailure, format!("compress 失败:, {}", e)))?;

    let ImageCompress {
        ext,
        before_size,
        after_size,
        rate,
        compressed_image,
        ..
    } = info;

    let info = CompressInfo {
        ext: Kind::from(ext),
        before_size: BigInt::from(before_size as u64),
        after_size: BigInt::from(after_size as u64),
        rate,
        compressed_image: Buffer::from(compressed_image),
    };

    Ok(info)
    // Ok(info)
}
