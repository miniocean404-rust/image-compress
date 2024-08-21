use clap::{parser, CommandFactory, Parser};
use cli::options::CompressOptions;
use indoc::indoc;

// 运行：cargo r -p cli --
fn main() {
    let after_help = indoc! {
        r#"
        List of supported codecs

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

        List of supported preprocessing options

        - Resize
        - Quantization
        - Alpha premultiply
    "#
    };

    let a = CompressOptions::command()
        .arg_required_else_help(true)
        .after_help(after_help);

    let arg = CompressOptions::parse();
    dbg!(arg);
}
