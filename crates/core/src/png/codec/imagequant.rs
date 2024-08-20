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

        // 将量化后的图像数据转换为 ImageBuffer
        // let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        // for (i, pixel) in pixels.iter().enumerate() {
        //     let x = (i as u32) % width;
        //     let y = (i as u32) / width;
        //     let color = palette[*pixel as usize];
        //     imgbuf.put_pixel(x, y, Rgba([color.r, color.g, color.b, color.a]));
        // }

        // // 将 ImageBuffer 编码为 PNG 并保存到文件
        // let file = File::create("output.png")?;
        // let ref mut w = BufWriter::new(file);
        // imgbuf.write_to(w, image::ImageOutputFormat::Png)?;

        Ok(vec![])
    }
}

// impl From<RGBA> for image::Rgba<u8> {
//     fn from(rgba: RGBA) -> Self {
//         image::Rgba([rgba.r, rgba.g, rgba.b, rgba.a])
//     }
// }
