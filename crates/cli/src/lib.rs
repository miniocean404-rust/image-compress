use clap::Parser;
use commands::{png::codec::PNGCodecSubCommand, Command, CommandRunner, CompressOptions};

mod commands;

pub fn run() -> anyhow::Result<()> {
    let parse = CompressOptions::parse();

    match &parse.command {
        Command::Png(PNGCodecSubCommand::Oxipng(options)) => options.execute(&parse),
        Command::Png(PNGCodecSubCommand::Imagequant(options)) => options.execute(&parse),
        Command::Jpeg(options) => options.execute(&parse),
    }
}
