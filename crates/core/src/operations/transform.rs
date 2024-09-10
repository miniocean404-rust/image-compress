use anyhow::anyhow;
use image::DynamicImage;
use webp::Encoder;

pub struct ImageFormatTransformm {}

impl ImageFormatTransformm {
    pub fn transform(&self, buffer: &[u8]) -> anyhow::Result<Vec<u8>> {
        // 打开并解码图像
        // let image_buffer = ImageReader::open(input_path).map_err(|e| format!("失败的打开图片: {}", e))?;
        // let image = image_buffer.decode().map_err(|e| format!("失败的解码图片: {}\n", e))?;

        let image = image::load_from_memory(buffer)?;
        self.encode2webp(&image).map_err(|e| anyhow!(e.to_string()))
    }

    /// 将' DynamicImage '编码为webp格式的字节
    pub fn encode2webp(&self, image: &DynamicImage) -> Result<Vec<u8>, String> {
        let encoder =
            Encoder::from_image(image).map_err(|e| format!("失败的创建 WebP 编码器: {}", e))?;
        let webp_data = encoder.encode(100.0);
        Ok(webp_data.to_vec())
    }
}
