use image::{DynamicImage, GenericImageView, ImageReader};

pub type Options = oxipng::Options;

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
        Ok(oxipng::optimize_from_memory(mem.as_slice(), &self.options)?)
    }

    pub fn encode(image: &DynamicImage) {
        let (width, height) = image.dimensions();
        let color_space = image.color();
    }

    // fn supported_colorspaces(&self) -> &'static [ColorSpace] {
    //     &[ColorSpace::Luma, ColorSpace::LumaA, ColorSpace::RGB, ColorSpace::RGBA]
    // }

    // fn format(&self) -> ImageFormat {
    //     ImageFormat::PNG
    // }

    // fn supported_bit_depth(&self) -> &'static [BitDepth] {
    //     &[BitDepth::Eight, BitDepth::Sixteen]
    // }

    // fn default_depth(&self, depth: BitDepth) -> BitDepth {
    //     match depth {
    //         BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
    //         _ => BitDepth::Eight,
    //     }
    // }
}
