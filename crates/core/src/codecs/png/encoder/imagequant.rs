use std::io::Cursor;

use imagequant::RGBA;
use zune_core::{bit_depth::BitDepth, bytestream::ZWriter, colorspace::ColorSpace, options::DecoderOptions};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

use crate::codecs::png::encoder::imagequant_options::ImageQuantOptions;
use crate::error::{CompressError, Result};

// 有损压缩 lossy
#[derive(Default, Debug)]
pub struct ImageQuantEncoder {
    pub options: ImageQuantOptions,
    pub origin_size: usize,
    pub compress_size: usize,
}

impl ImageQuantEncoder {
    pub fn new() -> ImageQuantEncoder {
        ImageQuantEncoder::default()
    }

    pub fn new_with_options(options: ImageQuantOptions) -> ImageQuantEncoder {
        ImageQuantEncoder {
            options,
            ..ImageQuantEncoder::default()
        }
    }

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> Result<Vec<u8>> {
        let cursor = Cursor::new(buf);

        let image = Image::read(cursor, DecoderOptions::default()).map_err(CompressError::png_decode)?;

        let mut compress_buf = Cursor::new(vec![]);

        self.encode(&image, &mut compress_buf)
            .map_err(CompressError::png_encode)?;

        Ok(compress_buf.into_inner())
    }

    fn image_quant_encode<VecRGBA>(&self, data: VecRGBA, width: usize, height: usize) -> Result<Vec<u8>>
    where
        VecRGBA: Into<Box<[RGBA]>>,
    {
        let mut attr = imagequant::new();
        let mut img = attr
            .new_image(data, width, height, 0.0)
            .map_err(|e| CompressError::png_encode(format!("创建图像失败: {}", e)))?;

        attr.set_speed(self.options.speed)
            .map_err(|e| CompressError::invalid_parameter(format!("无效的速度参数: {}", e)))?;

        attr.set_quality(self.options.min_quality, self.options.max_quality)
            .map_err(|e| CompressError::invalid_parameter(format!("无效的质量参数: {}", e)))?;
        attr.set_last_index_transparent(self.options.last_index_transparent);
        // 要忽略的最低有效位数
        attr.set_min_posterization(self.options.min_posterization)
            .map_err(|e| CompressError::invalid_parameter(format!("无效的色调分离参数: {}", e)))?;

        // 为图像生成调色板
        let mut quantize_res = attr
            .quantize(&mut img)
            .map_err(|e| CompressError::quantize(format!("量化失败: {}", e)))?;
        // 设置图片抖动
        quantize_res
            .set_dithering_level(self.options.dithering)
            .map_err(|e| CompressError::invalid_parameter(format!("无效的抖动参数: {}", e)))?;
        // 颜色从输入 Gamma 转换为此 Gamma
        quantize_res
            .set_output_gamma(self.options.gamma)
            .map_err(|e| CompressError::invalid_parameter(format!("无效的 Gamma 参数: {}", e)))?;

        let (_palette, pixels) = quantize_res
            .remapped(&mut img)
            .map_err(|e| CompressError::png_encode(format!("重映射失败: {}", e)))?;

        // 获取调色板并用新像素覆盖以前的像素，也可以使用 remapped 获取调色板
        let palette = quantize_res.palette();

        let mut enc = lodepng::Encoder::new();
        enc.info_raw_mut().set_bitdepth(8);
        enc.set_palette(palette)
            .map_err(|e| CompressError::png_encode(format!("设置调色板失败: {}", e)))?;
        enc.encode(pixels.as_slice(), width, height)
            .map_err(|e| CompressError::png_encode(format!("lodepng 编码失败: {}", e)))
    }

    // 将 vec_data 转换为 RGBA 格式
    fn convert_to_rgba(&self, buffer: Vec<u8>) -> std::result::Result<Vec<RGBA>, &'static str> {
        if buffer.len() % 4 != 0 {
            return Err("buffer length is not a multiple of 4");
        }

        let rgba_data: Vec<RGBA> = buffer
            .chunks(4)
            .map(|chunk| RGBA {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            })
            .collect();

        Ok(rgba_data)
    }
}

impl EncoderTrait for ImageQuantEncoder {
    fn name(&self) -> &'static str {
        "imagequant"
    }

    fn encode_inner<T: zune_core::bytestream::ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> std::result::Result<usize, ImageErrors> {
        let (width, height) = image.dimensions();

        // 确保图片是 RGBA 色彩空间，imagequant 只支持 RGBA
        let image = if image.colorspace() != ColorSpace::RGBA {
            let mut img_clone = image.clone();
            img_clone.convert_color(ColorSpace::RGBA)?;
            img_clone
        } else {
            image.clone()
        };

        let vec_data = if image.depth() == BitDepth::Eight {
            // 如果是 8 个字节就拍平
            image.flatten_frames::<u8>()
        } else if image.depth() == BitDepth::Sixteen {
            // 如果是 16 个字节就转为 本机字节序
            image
                .frames_ref()
                .iter()
                .map(|frame| frame.u16_to_native_endian())
                .collect()
        } else {
            return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!(
                "不支持的位深度: {:?}",
                image.depth()
            ))));
        }
        .into_iter()
        .next()
        .ok_or_else(|| ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors("图像帧数据为空".to_string())))?;

        let mut writer = ZWriter::new(sink);

        // 验证数据长度是否正确（width * height * 4 字节 = RGBA）
        let expected_len = width * height * 4;
        if vec_data.len() != expected_len {
            return Err(ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!(
                "像素数据长度不匹配: 期望 {} 字节 ({}x{}x4), 实际 {} 字节",
                expected_len,
                width,
                height,
                vec_data.len()
            ))));
        }

        let data = self.convert_to_rgba(vec_data)?;

        let result = self
            .image_quant_encode(data, width, height)
            .map_err(|e| ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string())))?;

        writer
            .write(&result)
            .map_err(|e| ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}"))))?;

        Ok(writer.bytes_written())
    }

    fn supported_colorspaces(&self) -> &'static [zune_core::colorspace::ColorSpace] {
        &[ColorSpace::RGBA]
    }

    fn format(&self) -> zune_image::codecs::ImageFormat {
        ImageFormat::PNG
    }

    fn supported_bit_depth(&self) -> &'static [zune_core::bit_depth::BitDepth] {
        &[BitDepth::Eight, BitDepth::Sixteen]
    }

    fn default_depth(&self, depth: zune_core::bit_depth::BitDepth) -> zune_core::bit_depth::BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}
