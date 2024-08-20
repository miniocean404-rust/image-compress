use image::{DynamicImage, GenericImageView};
use imagequant::{Attributes, RGBA};

#[derive(Default)]
pub struct ImageQuantEncoder {
    pub options: Attributes,
    pub origin_size: usize,
    pub compress_size: usize,
}

impl ImageQuantEncoder {
    pub fn new() -> ImageQuantEncoder {
        ImageQuantEncoder::default()
    }

    pub fn encode(&self, image: &DynamicImage) -> anyhow::Result<Vec<u8>> {
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

        let mut quant = imagequant::new();
        let mut img = quant.new_image(data, width as usize, height as usize, 0.0)?;

        quant.set_speed(4)?;
        quant.set_quality(0, 100)?;
        quant.set_max_colors(128)?;

        let mut quantize_res = quant.quantize(&mut img)?;
        let (palette, pixels) = quantize_res.remapped(&mut img)?;

        Ok(vec![])
    }
}
