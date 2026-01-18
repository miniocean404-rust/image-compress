use image_compress::{compress::ImageCompress, Options};
use napi::bindgen_prelude::*;
use napi_derive::napi;

use super::options::{
    mozjpeg::NapiMozJpegOptions, oxipng::NapiOxiPngOptions, ravif::NapiAvifOptions,
    webp::NapiWebPOptions,
};

#[napi(object)]
pub struct CompressResult {
    /// 压缩后的图片数据
    pub compressed_image: Buffer,
    /// 压缩前文件大小 (字节)
    pub before_size: u32,
    /// 压缩后文件大小 (字节)
    pub after_size: u32,
    /// 压缩率 (百分比，正值表示体积减小)
    pub rate: f64,
}

/// 压缩图片
///
/// ### 参数
///
/// * `image_path` - 图片文件路径或图片 Buffer
/// * `options` - 可选的压缩选项，根据图片格式自动选择
///
/// ### 返回
///
/// 返回压缩结果，包含压缩后的图片数据和统计信息
#[napi]
pub fn compress(image: Either<String, Buffer>, options: Option<Object>) -> Result<CompressResult> {
    // 读取图片数据
    let image_data = match image {
        Either::A(path) => std::fs::read(&path).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("读取图片文件失败: {}", e),
            )
        })?,
        Either::B(buffer) => buffer.to_vec(),
    };

    // 创建压缩器
    let mut compressor = ImageCompress::new().with_buffer(image_data);

    // 根据选项设置压缩参数
    if let Some(opts) = options {
        // 根据图片类型选择对应的选项类型
        let compress_options = match compressor.ext {
            image_compress::SupportedFileTypes::Png => {
                Options::OxiPng(NapiOxiPngOptions::from(opts).into())
            }
            image_compress::SupportedFileTypes::Jpeg => {
                Options::MozJpeg(NapiMozJpegOptions::from(opts).into())
            }
            image_compress::SupportedFileTypes::WebP => {
                Options::WebP(NapiWebPOptions::from(opts).into())
            }
            image_compress::SupportedFileTypes::Avif => {
                Options::Avif(NapiAvifOptions::from(opts).into())
            }
            image_compress::SupportedFileTypes::Gif => Options::Gif(Default::default()),
            _ => {
                return Err(Error::new(
                    Status::InvalidArg,
                    "不支持的图片格式或无法识别图片类型",
                ))
            }
        };

        compressor = compressor.with_options(compress_options);
    } else {
        // 没有提供选项，使用默认选项
        let default_options = match compressor.ext {
            image_compress::SupportedFileTypes::Png => Options::OxiPng(Default::default()),
            image_compress::SupportedFileTypes::Jpeg => Options::MozJpeg(Default::default()),
            image_compress::SupportedFileTypes::WebP => Options::WebP(Default::default()),
            image_compress::SupportedFileTypes::Avif => Options::Avif(Default::default()),
            image_compress::SupportedFileTypes::Gif => Options::Gif(Default::default()),
            _ => {
                return Err(Error::new(
                    Status::InvalidArg,
                    "不支持的图片格式或无法识别图片类型",
                ))
            }
        };

        compressor = compressor.with_options(default_options);
    }

    // 执行压缩
    compressor.compress().map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("图片压缩失败: {}", e),
        )
    })?;

    Ok(CompressResult {
        compressed_image: compressor.compressed_image.into(),
        before_size: compressor.before_size as u32,
        after_size: compressor.after_size as u32,
        rate: compressor.rate,
    })
}

