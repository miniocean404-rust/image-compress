use clap::Parser;
use cli::options::CompressOptions;

// 运行：cargo r -p cli --
fn main() {
    let arg = CompressOptions::parse();
    dbg!(arg);
}
