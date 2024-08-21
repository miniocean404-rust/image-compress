use crate::commands::{CommandRunner, CompressOptions};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ImagequantOptions {
    /// imagequant 默认最小值 0
    #[arg(long = "min")]
    min_quality: Option<u8>,

    /// imagequant 默认最大值 100, 如果 max 小于 100，库将尝试使用更少的颜色。 颜色较少的图像并不总是较小，因为它会导致抖动增加。
    /// 如果最小值为 0 最大值为 100 那么它会在 100 的基础上尽力压缩
    #[arg(long = "max", default_value_t = 100)]
    max_quality: u8,

    /// 1 - 10
    /// 较快的速度会生成较低质量的图像，但对于实时生成图像可能很有用
    /// default: 4
    #[arg(short, long)]
    speed: Option<i32>,

    /// 要忽略的最低有效 bit 的数目。用于生成 VGA, 15 位纹理或其他复古平台的调色板，越大压缩的越小
    /// 0 - 4
    #[arg(long)]
    min_posterization: Option<u8>,

    /// 设置为 1.0 以获得平滑的图像，越小压缩越小
    #[arg(short, long)]
    dithering: Option<f32>,

    /// 默认值是sRGB (~1/2.2)，，越小压缩越小，不知道做什么的
    /// 0 - 1
    #[arg(short, long)]
    gamma: Option<f64>,

    /// 将透明颜色移动到调色板中的最后一个条目
    /// 这对于PNG来说效率较低，但某些有缺陷的软件却需要这样做
    /// true 会增大大小
    #[arg(short, long("last"))]
    last_index_transparent: Option<bool>,
}

impl CommandRunner for ImagequantOptions {
    fn execute(&self, _compress_options: &CompressOptions) -> anyhow::Result<()> {
        let file = &_compress_options.entry_file;

        if let Some(_file) = file {
            // let image = image::open(file)?;

            // let encoder = ImageQuantEncoder::new();
            // let lossy_vec = encoder.encode(&image).unwrap();

            // println!("压缩后字节数: {}", lossy_vec.len());
        }

        Ok(())
    }
}
