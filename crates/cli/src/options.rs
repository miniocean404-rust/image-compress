use clap::arg;
use clap::{command, Parser};

use crate::sub_commands::png::PNGCodecCommand;

// cargo run -- --name Mineral --files a b --files c --debug --set setted --test --test
// cargo run -- --help 获取帮助信息
#[derive(Parser, Debug)]
#[command(name="图片压缩" ,author = "MiniOcean404", version = "1.0.0", about = "支持 png", long_about = None)]
#[command(propagate_version = true)] // 为了 --version 在所有子命令中也有效
#[command(next_line_help = true)] // 更改展示方式为两行
pub struct CompressOptions {
    /// 压缩 png 图片的选项
    #[arg(short = 'p', long = "png")]
    png: String,

    // clap::ArgAction::Append 多次执行的值叠加
    /// 这是一串 file 的名字
    // #[arg(short, long, num_args = 1.., action = ArgAction::Append)]
    // files: Vec<String>,

    /// 这是一个带默认参数的 count
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,

    /// 可选的操作
    // name1: Option<String>,

    /// 隐式的 Action Set
    // #[arg(short, long, value_name = "File Path")]
    // info: Option<PathBuf>,

    /// --test 执行次数
    // #[arg(short, long, action = ArgAction::Count)]
    // test: u8,

    /// clap::ArgAction::Set 只能设置一次
    // #[arg(short, long, default_value_t = String::from(""), action = ArgAction::Set)]
    // set: String,

    #[command(subcommand)]
    command: Option<PNGCodecCommand>,
}
