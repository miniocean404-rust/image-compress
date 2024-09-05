use image_compress::export::{self};
use std::fs;

use image_compress::compress::{ImageCompress, Options};
use image_compress::support::SupportedFileTypes;
use napi::bindgen_prelude::*;
use napi_derive::napi;

use super::kind::Kind;
use super::options::mozjpeg::NapiMozJpegOptions;
use super::options::oxipng::NapiOxiPngOptions;
use super::options::ravif::NapiAvifOptions;
use super::options::webp::NapiWebPOptions;

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
    #[napi(
        ts_arg_type = "MozJpegOptions | OxiPngOptions | ImageQuantOptions | WebPOptions | AvifOptions"
    )]
    options: Object,
) -> Result<CompressInfo> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    let buffer = fs::read(file)?;

    let base_info = ImageCompress::new().with_buffer(buffer);

    let mut info = match base_info.ext {
        SupportedFileTypes::Jpeg => {
            let js_options = NapiMozJpegOptions::from(options);
            let options = export::MozJpegOptions::from(js_options);
            base_info.with_options(Options::MozJpeg(options))
        }
        SupportedFileTypes::Png => {
            let js_options = NapiOxiPngOptions::from(options);
            let options = export::OxiPngOptions::from(js_options);
            base_info.with_options(Options::OxiPng(options))
        }
        SupportedFileTypes::WebP => {
            let js_options = NapiWebPOptions::from(options);
            let options = export::WebPOptions::from(js_options);
            base_info.with_options(Options::WebP(options))
        }
        SupportedFileTypes::Avif => {
            let js_options = NapiAvifOptions::from(options);
            let options = export::AvifOptions::from(js_options);
            base_info.with_options(Options::Avif(options))
        }
        SupportedFileTypes::Unknown => {
            return Err(Error::new(
                Status::GenericFailure,
                "不支持的类型".to_string(),
            ))
        }
    };

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
}
