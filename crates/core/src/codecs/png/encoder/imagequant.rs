use std::io::Cursor;

use anyhow::anyhow;
use imagequant::RGBA;
use zune_core::{
    bit_depth::BitDepth, bytestream::ZWriter, colorspace::ColorSpace, options::DecoderOptions,
};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

use crate::codecs::png::encoder::imagequant_options::ImageQuantOptions;

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

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);

        let image = Image::read(cursor, DecoderOptions::default())?;

        let mut compress_buf = Cursor::new(vec![]);
        self.encode(&image, &mut compress_buf)?;

        Ok(compress_buf.into_inner())
    }

    fn image_quant_encode<VecRGBA>(
        &self,
        data: VecRGBA,
        width: usize,
        height: usize,
    ) -> anyhow::Result<Vec<u8>>
    where
        VecRGBA: Into<Box<[RGBA]>>,
    {
        let mut attr = imagequant::new();
        let mut img = attr.new_image(data, width, height, 0.0)?;

        attr.set_speed(self.options.speed)?;
        attr.set_quality(self.options.min_quality, self.options.max_quality)?;
        attr.set_last_index_transparent(self.options.last_index_transparent);
        // 要忽略的最低有效位数
        attr.set_min_posterization(self.options.min_posterization)?;

        // 为图像生成调色板
        let mut quantize_res = attr.quantize(&mut img)?;
        // 设置图片抖动
        quantize_res.set_dithering_level(self.options.dithering)?;
        // 颜色从输入 Gamma 转换为此 Gamma
        quantize_res.set_output_gamma(self.options.gamma)?;
        let (_palette, pixels) = quantize_res.remapped(&mut img)?;
        // 获取调色板并用新像素覆盖以前的像素，也可以使用 remapped 获取调色板
        let palette = quantize_res.palette();

        let mut enc = lodepng::Encoder::new();
        enc.info_raw_mut().set_bitdepth(8);
        enc.set_palette(palette)?;
        enc.encode(pixels.as_slice(), width, height)
            .map_err(|e| anyhow!(e.to_string()))
    }

    // 将 vec_data 转换为 RGBA 格式
    fn convert_to_rgba(&self, buffer: Vec<u8>) -> Result<Vec<RGBA>, &'static str> {
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
    ) -> Result<usize, ImageErrors> {
        let colorspace = image.colorspace();
        let (width, height) = image.dimensions();

        let vec_data = if image.depth() == BitDepth::Eight {
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

        let mut writer = ZWriter::new(sink);

        let data = self.convert_to_rgba(vec_data)?;

        let result = self.image_quant_encode(data, width, height).map_err(|e| {
            ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string()))
        })?;

        writer.write(&result).map_err(|e| {
            ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
        })?;

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

    fn default_depth(
        &self,
        depth: zune_core::bit_depth::BitDepth,
    ) -> zune_core::bit_depth::BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}
