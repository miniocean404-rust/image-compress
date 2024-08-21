pub mod jpeg;
pub mod png;

use std::path::PathBuf;

use clap::Subcommand;
use clap::{command, Parser};
use indoc::indoc;
use jpeg::JpegCodecOptions;

use crate::commands::png::codec::PNGCodecSubCommand;

#[derive(Parser, Debug)]
#[command(
    name = "图片压缩",
    author = "MiniOcean404",
    version = "1.0.0",
    about = "这是一个图片压缩的 cli 工具",
    long_about = "这是一个图片压缩的 cli 工具, 支持多种编码格式的图片压缩"
)]
#[command(propagate_version = true)] // 为了 --version 在所有子命令中也有效
#[command(next_line_help = true)] // 更改展示方式为两行
#[command(arg_required_else_help = true)]
#[command(after_help = indoc! {
r#"
支持的编码 codecs:

    | Image Format  | Input | Output | Note            |
    | ------------- | ----- | ------ | --------------- |
    | avif          | O     | O      | Static only     |
    | bmp           | O     | X      |                 |
    | farbfeld      | O     | O      |                 |
    | hdr           | O     | O      |                 |
    | jpeg          | O     | O      |                 |
    | jpeg_xl(jxl)  | O     | O      |                 |
    | mozjpeg(moz)  | O     | O      |                 |
    | oxipng(oxi)   | O     | O      | Static only     |
    | png           | O     | O      | Static only     |
    | ppm           | O     | O      |                 |
    | psd           | O     | X      |                 |
    | qoi           | O     | O      |                 |
    | webp          | O     | O      | Static only     |

支持的预处理选项:

    - Resize
    - Quantization
    - Alpha premultiply
"#})]
pub struct CompressOptions {
    /// 压缩的文件路径
    #[clap(long, short = 'f', group = "input")]
    filename: Option<PathBuf>,

    /// 压缩的文件路径
    #[clap(long, group = "input")]
    entry_dir: Option<PathBuf>,

    /// 输出文件路径
    #[clap(long, group = "output")]
    out_file: Option<PathBuf>,

    /// 输出目录
    #[clap(long, group = "output")]
    out_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 是否需要压缩 Png 图片
    #[command(subcommand)]
    Png(PNGCodecSubCommand),

    /// 是否需要压缩 jpeg 图片
    Jpeg(JpegCodecOptions),
}

pub trait CommandRunner {
    fn execute(&self) -> anyhow::Result<()>;
}
