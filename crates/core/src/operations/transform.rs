use image::DynamicImage;
pub use image::ImageFormat;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct ImageFormatTransform {
    pub origin: Vec<u8>,
    pub origin_format: ImageFormat,
    pub after: Vec<u8>,
    pub after_format: ImageFormat,
}

impl ImageFormatTransform {
    // 打开并解码图像
    // let image_buffer = ImageReader::open(input_path).map_err(|e| format!("失败的打开图片: {}", e))?;
    // let image = image_buffer.decode().map_err(|e| format!("失败的解码图片: {}\n", e))?;

    pub fn new(buffer: Vec<u8>, format: ImageFormat) -> Self {
        let origin_format = image::guess_format(&buffer).expect("无法解析的格式");

        Self {
            origin: buffer,
            origin_format,
            after: vec![],
            after_format: format,
        }
    }

    pub fn transform(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut image = image::load_from_memory(&self.origin)?;

        if self.after_format == ImageFormat::Jpeg {
            let buffer = image.to_rgb8();
            image = DynamicImage::ImageRgb8(buffer);
        }

        let mut bytes: Cursor<Vec<u8>> = Cursor::new(vec![]);
        image.write_to(&mut bytes, self.after_format)?;

        self.after = bytes.into_inner();

        Ok(self.after.clone())
    }
}
