use image::ImageFormat;
use std::io::Cursor;

pub struct ImageFormatTransform {}

impl ImageFormatTransform {
    // 打开并解码图像
    // let image_buffer = ImageReader::open(input_path).map_err(|e| format!("失败的打开图片: {}", e))?;
    // let image = image_buffer.decode().map_err(|e| format!("失败的解码图片: {}\n", e))?;

    pub fn transform(&self, buffer: &[u8], format: ImageFormat) -> anyhow::Result<Vec<u8>> {
        let image = image::load_from_memory(buffer)?;

        let mut bytes: Cursor<Vec<u8>> = Cursor::new(vec![]);
        image.write_to(&mut bytes, format)?;

        Ok(bytes.into_inner())
    }
}
