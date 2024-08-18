use oxipng::PngError;

use zune_core::{
    bit_depth::BitDepth,
    bytestream::{ZByteWriterTrait, ZWriter},
    colorspace::ColorSpace,
};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

/// [`oxipng::Options`] 别名
pub type OxiPngOptions = oxipng::Options;

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

    pub fn optimize_from_memory(&self, mem: &Vec<u8>) -> Result<Vec<u8>, PngError> {
        // oxipng_options.deflate = Zopfli {
        //     iterations: NonZeroU8::new(15).ok_or("")?,
        // };
        // let mut oxipng_options = Options::from_preset(level);
        // oxipng_options.deflate = Libdeflater { compression: 6 };
        // oxipng_options.interlace = Some(Interlacing::None);
        oxipng::optimize_from_memory(mem.as_slice(), &self.options)
    }

    #[cfg(feature = "filesystem")]
    pub fn to_file(&self, input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        use std::fs::File;
        use std::io::Write;

        // let options = Options::max_compression(); // 设置压缩级别，范围是 0 到 6

        // let input = PathBuf::from(input);
        // let output = PathBuf::from(output);
        //
        // optimize(
        //     &InFile::Path(input),
        //     &OutFile::Path {
        //         path: Some(output),
        //         preserve_attrs: false, // 是否保留属性
        //     },
        //     &options,
        // )?;

        let in_file = fs::read(input)?;
        let png_vec = self.optimize_from_memory(&in_file)?;

        // 写入文件
        let mut output_file = File::create(output)?;
        output_file.write_all(png_vec.as_slice())?;

        Ok(())
    }
}

impl EncoderTrait for OxiPngEncoder {
    fn name(&self) -> &'static str {
        "oxipng"
    }

    fn encode_inner<T: ZByteWriterTrait>(&mut self, image: &Image, sink: T) -> Result<usize, ImageErrors> {
        // 获取图片宽高
        let (width, height) = image.dimensions();

        // inlined `to_u8` method because its private
        let colorspace = image.colorspace();

        let data = if image.depth() == BitDepth::Eight {
            // 如果是 8 个字节就拍平
            image.flatten_frames::<u8>()
        } else if image.depth() == BitDepth::Sixteen {
            // 如果是 16 个字节就转为 本机字节序
            image.frames_ref().iter().map(|frame| frame.u16_to_native_endian(colorspace)).collect()
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
                ColorSpace::Luma => oxipng::ColorType::Grayscale { transparent_shade: None },
                ColorSpace::RGB => oxipng::ColorType::RGB { transparent_color: None },
                ColorSpace::LumaA => oxipng::ColorType::GrayscaleAlpha,
                ColorSpace::RGBA => oxipng::ColorType::RGBA,
                cs => {
                    return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::UnsupportedColorspace(
                        cs,
                        self.supported_colorspaces(),
                    )))
                }
            },
            match image.depth() {
                BitDepth::Eight => oxipng::BitDepth::Eight,
                BitDepth::Sixteen => oxipng::BitDepth::Sixteen,
                d => return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::Generic(format!("{d:?} 字节深度不支持")))),
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

        writer
            .write(&result)
            .map_err(|e| ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}"))))?;

        Ok(writer.bytes_written())
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::Luma, ColorSpace::LumaA, ColorSpace::RGB, ColorSpace::RGBA]
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
