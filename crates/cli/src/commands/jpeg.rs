use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct JpegCodecOptions {
    /// mozjpeg 压缩 jpeg 图片的选项
    #[arg(short, long, value_enum, default_value_t = PluginTargetType::Jpeg)]
    ext: PluginTargetType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum PluginTargetType {
    /// Jpeg 文件后缀
    Jpeg,

    /// Jpg 文件后缀
    Jpg,
}

impl super::CommandRunner for JpegCodecOptions {
    fn execute(&self) -> anyhow::Result<()> {
        println!("JpegCodecOptions");
        Ok(())
    }
}
