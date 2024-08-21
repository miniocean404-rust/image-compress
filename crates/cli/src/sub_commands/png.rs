use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PNGCodecCommand {
    /// Oxipng 无损压缩编码
    Oxipng {
        #[arg(short, long, default_value = "")]
        name: Option<String>,
    },

    /// ImageQuant 有损压缩编码 (把 png24 压缩成 png8)
    Imagequant {
        #[arg(short, long, default_value = "")]
        name: Option<String>,
    },
}
