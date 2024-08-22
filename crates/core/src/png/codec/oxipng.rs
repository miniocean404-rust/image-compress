use anyhow::anyhow;
use image::{DynamicImage, GenericImageView};

pub type Options = oxipng::Options;

// 无损压缩 lossless
#[derive(Debug, Default)]
pub struct OxiPngEncoder {
    pub options: Options,
    pub origin_size: usize,
    pub compress_size: usize,
}

impl OxiPngEncoder {
    pub fn new() -> OxiPngEncoder {
        OxiPngEncoder::default()
    }

    pub fn new_with_options(options: Options) -> OxiPngEncoder {
        OxiPngEncoder {
            options,
            ..OxiPngEncoder::default()
        }
    }

    pub fn compress_with_mem(&self, mem: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // oxipng_options.deflate = Zopfli {
        //     iterations: NonZeroU8::new(15).ok_or("")?,
        // };
        // let mut oxipng_options = Options::from_preset(level);
        // oxipng_options.deflate = Libdeflater { compression: 6 };
        // oxipng_options.interlace = Some(Interlacing::None);

        // let options = Options::max_compression(); // 设置压缩级别，范围是 0 到 6
        Ok(oxipng::optimize_from_memory(mem.as_slice(), &self.options)?)
    }

    #[cfg(feature = "native")]
    pub fn compress_with_file(&self, input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        use oxipng::InFile;
        use oxipng::OutFile;
        use std::path::PathBuf;

        // let in_file = fs::read(input)?;
        // let png_vec = self.optimize_from_memory(&in_file)?;
        // 写入文件
        // let mut output_file = File::create(output)?;
        // output_file.write_all(png_vec.as_slice())?;

        let input = InFile::Path(PathBuf::from(input));
        let output = OutFile::from_path(PathBuf::from(output));
        oxipng::optimize(&input, &output, &self.options).map_err(|e| return Err(anyhow!("压缩失败: {}", e.to_string())))?;

        Ok(())
    }

    pub fn encode(&self, buffer: &[u8]) -> anyhow::Result<Vec<u8>> {
        let image = image::load_from_memory(buffer)?;

        let (width, height) = image.dimensions();
        let color_space = image.color();

        let data = match image {
            // DynamicImage::ImageLuma8(img) => img.to_vec(),
            // DynamicImage::ImageLumaA8(img) => img.to_vec(),
            // DynamicImage::ImageRgb8(img) => img.to_vec(),
            // DynamicImage::ImageRgba8(img) => img.to_vec(),
            // DynamicImage::ImageLuma16(img) => img.to_vec().iter().flat_map(|&num| num.to_ne_bytes()).collect(),
            // DynamicImage::ImageLumaA16(img) => img.to_vec().iter().flat_map(|&num| num.to_ne_bytes()).collect(),
            // DynamicImage::ImageRgb16(img) => img.to_vec().iter().flat_map(|&num| num.to_ne_bytes()).collect(),
            // DynamicImage::ImageRgba16(img) => img.to_vec().iter().flat_map(|&num| num.to_ne_bytes()).collect(),
            // _ => return Err(anyhow!("不支持的颜色空间")),
            DynamicImage::ImageRgb32F(_) => return Err(anyhow!("不支持的颜色空间")),
            DynamicImage::ImageRgba32F(_) => return Err(anyhow!("不支持的颜色空间")),
            img => img.clone().into_bytes(),
        };

        #[allow(unused_mut)]
        let mut img = oxipng::RawImage::new(
            width,
            height,
            match color_space {
                image::ColorType::L8 | image::ColorType::L16 => oxipng::ColorType::Grayscale { transparent_shade: None },
                image::ColorType::La8 | image::ColorType::La16 => oxipng::ColorType::GrayscaleAlpha,
                image::ColorType::Rgb8 | image::ColorType::Rgb16 => oxipng::ColorType::RGB { transparent_color: None },
                image::ColorType::Rgba8 | image::ColorType::Rgba16 => oxipng::ColorType::RGBA,
                cs => {
                    return Err(anyhow!(format!("{:?} 颜色空间不支持，支持的是 {:?}", cs, self.supported_colorspaces())));
                }
            },
            match color_space {
                image::ColorType::L8 | image::ColorType::La8 | image::ColorType::Rgb8 | image::ColorType::Rgba8 => oxipng::BitDepth::Eight,
                image::ColorType::L16 | image::ColorType::La16 | image::ColorType::Rgba16 | image::ColorType::Rgb16 => oxipng::BitDepth::Sixteen,
                d => return Err(anyhow!(format!("{:?} 字节深度不支持", d))),
            },
            data,
        )
        .map_err(|e| anyhow!(e.to_string()))?;

        let result = img.create_optimized_png(&self.options).map_err(|e| anyhow!(e.to_string()))?;

        Ok(result)
    }

    fn supported_colorspaces(&self) -> &'static [image::ColorType] {
        &[
            image::ColorType::L8,
            image::ColorType::L16,
            image::ColorType::La8,
            image::ColorType::La16,
            image::ColorType::Rgb8,
            image::ColorType::Rgb16,
            image::ColorType::Rgba8,
            image::ColorType::Rgba16,
        ]
    }

    pub fn format(&self) -> image::ImageFormat {
        image::ImageFormat::Png
    }

    pub fn supported_bit_depth(&self) -> &'static [image::ColorType] {
        &[
            image::ColorType::L8,
            image::ColorType::L16,
            image::ColorType::La8,
            image::ColorType::La16,
            image::ColorType::Rgb8,
            image::ColorType::Rgb16,
            image::ColorType::Rgba8,
            image::ColorType::Rgba16,
        ]
    }

    pub fn default_depth(&self, depth: image::ColorType) -> oxipng::BitDepth {
        match depth {
            image::ColorType::L16 | image::ColorType::La16 | image::ColorType::Rgba16 | image::ColorType::Rgb16 => oxipng::BitDepth::Sixteen,
            _ => oxipng::BitDepth::Eight,
        }
    }
}
