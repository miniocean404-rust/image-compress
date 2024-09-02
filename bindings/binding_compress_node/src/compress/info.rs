use std::fs;
use image_compress::export::*;

use image_compress::compress::{ImageCompress, OptionsTrait};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use image_compress::support::SupportedFileTypes;

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
    #[napi(
        ts_arg_type = "MozJpegOptions | OxiPngOptions | ImageQuantOptions | WebPOptions | AvifOptions"
    )]
    options: Object,
) -> Result<CompressInfo> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    let buffer = fs::read(file)?;

    let compressor: ImageCompress<dyn OptionsTrait> = ImageCompress::new(buffer);

    let mut info: ImageCompress<dyn OptionsTrait> = match compressor.ext {
        SupportedFileTypes::Jpeg => {
            let options: super::options::mozjpeg::MozJpegOptions = options.into();
            Ok(compressor.with_options(MozJpegOptions::from(options)))
        }
        SupportedFileTypes::Png => {
            let options: super::options::oxipng::OxiPngOptions = options.into();
            Ok(compressor.with_options(OxiPngOptions::from(options)))
        }
        SupportedFileTypes::WebP => {
            let options: super::options::webp::WebPOptions = options.into();
            Ok(compressor.with_options(WebPOptions::from(options)))
        }
        SupportedFileTypes::Avif => {
            let options: super::options::ravif::AvifOptions = options.into();
            Ok(compressor.with_options(AvifOptions::from(options)))
        }
        SupportedFileTypes::Unknown => Err(Error::new(Status::GenericFailure, "不支持的类型".to_string()))
    }?;


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
