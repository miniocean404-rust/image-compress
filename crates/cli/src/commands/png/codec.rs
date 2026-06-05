use clap::Subcommand;

use super::{imagequant::ImagequantOptions, oxipng::OxipngOptions};

#[derive(Subcommand, Debug)]
pub enum PNGCodecSubCommand {
    /// Oxipng 无损压缩编码
    Oxipng(OxipngOptions),

    /// ImageQuant 有损压缩编码 (把 png24 压缩成 png8)
    Imagequant(ImagequantOptions),
}
