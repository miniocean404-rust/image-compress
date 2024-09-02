use image_compress::export::{self, *};
use std::fs;

use image_compress::compress::{ImageCompress, OptionsTrait};
use image_compress::support::SupportedFileTypes;
use napi::bindgen_prelude::*;
use napi_derive::napi;

use super::kind::Kind;
use super::options;

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

    let compressor = ImageCompress::new(buffer);

    let options = match compressor.ext {
        SupportedFileTypes::Jpeg => {
            let options: super::options::mozjpeg::MozJpegOptions = options.into();
            Ok(options)
        }
        // SupportedFileTypes::Png => {
        //     let options: super::options::oxipng::OxiPngOptions = options.into();
        //     Some(options)
        // }
        // SupportedFileTypes::WebP => {
        //     let options: super::options::webp::WebPOptions = options.into();
        //     Some(options)
        // }
        // SupportedFileTypes::Avif => {
        //     let options: super::options::ravif::AvifOptions = options.into();
        //     Some(options)
        // }
        SupportedFileTypes::Unknown => Err(Error::new(
            Status::GenericFailure,
            "不支持的类型".to_string(),
        )),
        _ => Err(Error::new(
            Status::GenericFailure,
            "不支持的类型".to_string(),
        )),
    }?;

    let mut info = compressor.with_options(export::MozJpegOptions::from(options));

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
