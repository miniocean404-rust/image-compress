use std::{fs, path::Path};

use image::{DynamicImage, ImageReader};
use webp::Encoder;

/// 将' DynamicImage '编码为webp格式的字节
pub fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let encoder = Encoder::from_image(image).map_err(|e| format!("失败的创建 WebP 编码器: {}", e))?;
    let webp_data = encoder.encode(100.0);
    Ok(webp_data.to_vec())
}

fn convert_image(input_path: &str, output_dir: &Option<&str>) -> Result<(), String> {
    let input_path = Path::new(input_path);

    // 打开并解码图像
    let image_buffer = ImageReader::open(input_path).map_err(|e| format!("失败的打开图片: {}", e))?;
    let image = image_buffer.decode().map_err(|e| format!("失败的解码图片: {}\n", e))?;

    // 将图像编码为WebP
    let webp_data = encode_webp(&image)?;

    // 确定输出路径
    let output_path = if let Some(output_dir) = output_dir {
        Path::new(output_dir).join(input_path.file_stem().unwrap()).with_extension("webp")
    } else {
        input_path.with_extension("webp")
    };

    // 将WebP图像写入输出路径
    fs::write(output_path.clone(), webp_data).map_err(|e| format!("失败的去写入文件: {}", e))?;

    println!("生成: {}", output_path.display());
    Ok(())
}

fn to_webp() {
    let dir = ["./input/5613.png", "./input/coffee.jpg"];
    for file in dir.iter() {
        if let Err(e) = convert_image(file, &Some("./output")) {
            eprintln!("Error: {}", e);
        }
    }
}
