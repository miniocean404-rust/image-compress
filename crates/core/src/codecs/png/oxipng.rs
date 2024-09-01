use std::io::Cursor;

use zune_core::{
    bit_depth::BitDepth,
    bytestream::{ZByteWriterTrait, ZWriter},
    colorspace::ColorSpace,
    options::DecoderOptions,
};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

use crate::codecs::OptionsTrait;

/// [`oxipng::Options`] 别名
/// fix_errors: bool:               尝试在解码输入文件时修复错误，而不是返回 `Err`。默认值: `false`
/// force: bool:                    即使压缩没有改进，也写入输出。默认值: `false`
/// filter: IndexSet<RowFilter>:    尝试在文件上使用哪些 RowFilters。默认值: `None,Sub,Entropy,Bigrams`
/// interlace: Option<Interlacing>: 是否更改文件的交错类型。`None` 将不会更改当前的交错类型。`Some(x)` 将把文件更改为交错模式 `x`。默认值: `Some(Interlacing::None)`
/// optimize_alpha: bool:           是否允许更改透明像素以提高压缩率。
/// bit_depth_reduction: bool:      是否尝试位深度减少。默认值: `true`
/// color_type_reduction: bool:     是否尝试颜色类型减少。默认值: `true`
/// palette_reduction: bool:        是否尝试调色板减少。默认值: `true`
/// grayscale_reduction: bool:      是否尝试灰度减少。默认值: `true`
/// idat_recoding: bool:            是否对 IDAT 和其他压缩块进行重新编码。如果执行任何类型的减少，将无视此设置执行 IDAT 重新编码。默认值: `true`
/// scale_16: bool:                 是否强制将 16 位缩减为 8 位。默认值: `false`
/// strip: StripChunks:             从 PNG 文件中剥离哪些块（如果有的话）。默认值: `None`
/// deflate: Deflaters:             使用哪种 DEFLATE 算法。默认值: `Libdeflater`
/// fast_evaluation: bool:          是否使用快速评估来选择最佳过滤器。默认值: `true`
/// timeout: Option<Duration>:      优化的最大时间。如果超时，将跳过进一步的潜在优化。
pub type OxiPngOptions = oxipng::Options;

impl OptionsTrait for OxiPngOptions {}

//  无损压缩 lossless
/// OxiPNG 编码器
#[derive(Default, Debug)]
pub struct OxiPngEncoder {
    options: OxiPngOptions,
}

impl OxiPngEncoder {
    /// Create a new encoder
    pub fn new() -> OxiPngEncoder {
        OxiPngEncoder::default()
    }
    /// Create a new encoder with specified options
    pub fn new_with_options(options: OxiPngOptions) -> OxiPngEncoder {
        OxiPngEncoder { options }
    }

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);

        let image = Image::read(cursor, DecoderOptions::default())?;

        let mut compress_buf = Cursor::new(vec![]);
        self.encode(&image, &mut compress_buf)?;

        Ok(compress_buf.into_inner())
    }
}

impl EncoderTrait for OxiPngEncoder {
    fn name(&self) -> &'static str {
        "oxipng"
    }

    fn encode_inner<T: ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        // 获取图片宽高
        let (width, height) = image.dimensions();

        // inlined `to_u8` method because its private
        let colorspace = image.colorspace();

        let data = if image.depth() == BitDepth::Eight {
            // 如果是 8 个字节就拍平
            image.flatten_frames::<u8>()
        } else if image.depth() == BitDepth::Sixteen {
            // 如果是 16 个字节就转为 本机字节序
            image
                .frames_ref()
                .iter()
                .map(|frame| frame.u16_to_native_endian(colorspace))
                .collect()
        } else {
            unreachable!()
        }
        .into_iter()
        .next()
        .unwrap();

        #[allow(unused_mut)]
        let mut img = oxipng::RawImage::new(
            width as u32,
            height as u32,
            match image.colorspace() {
                ColorSpace::Luma => oxipng::ColorType::Grayscale {
                    transparent_shade: None,
                },
                ColorSpace::RGB => oxipng::ColorType::RGB {
                    transparent_color: None,
                },
                ColorSpace::LumaA => oxipng::ColorType::GrayscaleAlpha,
                ColorSpace::RGBA => oxipng::ColorType::RGBA,
                cs => {
                    return Err(ImageErrors::EncodeErrors(
                        ImgEncodeErrors::UnsupportedColorspace(cs, self.supported_colorspaces()),
                    ))
                }
            },
            match image.depth() {
                BitDepth::Eight => oxipng::BitDepth::Eight,
                BitDepth::Sixteen => oxipng::BitDepth::Sixteen,
                d => {
                    return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::Generic(
                        format!("{d:?} 字节深度不支持"),
                    )))
                }
            },
            data,
        )
        .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

        // #[cfg(feature = "metadata")]
        // {
        //     use exif::experimental::Writer;
        //
        //     let mut buf = std::io::Cursor::new(vec![]);
        //
        //     if let Some(fields) = &image.metadata().exif() {
        //         let mut writer = Writer::new();
        //
        //         for metadatum in *fields {
        //             writer.push_field(metadatum);
        //         }
        //         let result = writer.write(&mut buf, false);
        //         if result.is_ok() {
        //             img.add_png_chunk(*b"eXIf", buf.into_inner());
        //         } else {
        //             log::warn!("Writing exif failed {:?}", result);
        //         }
        //     }
        // }

        let mut writer = ZWriter::new(sink);

        let result = img
            .create_optimized_png(&self.options)
            .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

        writer.write(&result).map_err(|e| {
            ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
        })?;

        Ok(writer.bytes_written())
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[
            ColorSpace::Luma,
            ColorSpace::LumaA,
            ColorSpace::RGB,
            ColorSpace::RGBA,
        ]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::PNG
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight, BitDepth::Sixteen]
    }

    fn default_depth(&self, depth: BitDepth) -> BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}
