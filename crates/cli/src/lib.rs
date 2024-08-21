use clap::Parser;
use commands::{png::codec::PNGCodecSubCommand, Command, CommandRunner, CompressOptions};

mod commands;

pub fn run() -> anyhow::Result<()> {
    let command = CompressOptions::parse().command;

    match &command {
        Command::Png(PNGCodecSubCommand::Oxipng(options)) => options.execute(),
        Command::Png(PNGCodecSubCommand::Imagequant(options)) => options.execute(),
        Command::Jpeg(options) => options.execute(),
    }
}
