use image::GenericImageView;
use imagequant::RGBA;

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

    #[cfg(feature = "native")]
    pub fn encode_with_file() {
        // let mut encoder = lodepng::Encoder::new();
        // encoder.set_palette(palette.as_slice())?;

        // // 写入文件
        // encoder.encode_file(output, pixels.as_slice(), width, height)?;
    }

    pub fn encode(&self, buffer: &[u8]) -> anyhow::Result<Vec<u8>> {
        let image = image::load_from_memory(buffer)?;

        let (width, height) = image.dimensions();

        let mut data = Vec::with_capacity((width * height) as usize);

        for pixel in image.to_rgba8().pixels() {
            let rgba = RGBA {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            };
            data.push(rgba);
        }

        let mut attr = imagequant::new();
        let mut img = attr.new_image(data, width as usize, height as usize, 0.0)?;

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
        let png_vec = enc.encode(pixels.as_slice(), width as usize, height as usize)?;

        // 将量化后的图像数据转换为 ImageBuffer

        // 将 ImageBuffer 编码为 PNG 并保存到文件
        // let file = File::create("output.png")?;
        // let ref mut w = BufWriter::new(file);
        // imgbuf.write_to(w, image::ImageOutputFormat::Png)?;

        Ok(png_vec)
    }

    pub fn format(&self) -> image::ImageFormat {
        image::ImageFormat::Png
    }
}

// impl From<RGBA> for image::Rgba<u8> {
//     fn from(rgba: RGBA) -> Self {
//         image::Rgba([rgba.r, rgba.g, rgba.b, rgba.a])
//     }
// }
