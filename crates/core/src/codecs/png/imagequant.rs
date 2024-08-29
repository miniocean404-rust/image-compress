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

#[derive(Debug)]
pub struct ImageQuantOptions {
    // imagequant 默认最小值 0
    pub min_quality: u8,

    // imagequant 默认最大值 100, 如果 max 小于 100，库将尝试使用更少的颜色。 颜色较少的图像并不总是较小，因为它会导致抖动增加。
    // 如果最小值为 0 最大值为 100 那么它会在 100 的基础上尽力压缩
    pub max_quality: u8,

    // 1 - 10
    // 较快的速度会生成较低质量的图像，但对于实时生成图像可能很有用
    // default: 4
    pub speed: i32,

    // 要忽略的最低有效 bit 的数目。用于生成 VGA, 15 位纹理或其他复古平台的调色板，越大压缩的越小
    // 0 - 4
    pub min_posterization: u8,

    // 设置为 1.0 以获得平滑的图像，越小压缩越小
    pub dithering: f32,

    // 默认值是sRGB (~1/2.2)，，越小压缩越小，不知道做什么的
    // 0 - 1
    pub gamma: f64,

    // 将透明颜色移动到调色板中的最后一个条目
    // 这对于PNG来说效率较低，但某些有缺陷的软件却需要这样做
    // true 会增大大小
    pub last_index_transparent: bool,
}

impl Default for ImageQuantOptions {
    fn default() -> Self {
        ImageQuantOptions {
            min_quality: 0,
            max_quality: 100,
            speed: 1,
            min_posterization: 4,
            dithering: 0.0,
            gamma: 0.1,
            last_index_transparent: false,
        }
    }
}

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
