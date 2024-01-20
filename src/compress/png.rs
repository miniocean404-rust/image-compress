use std::{error::Error, path::PathBuf};

use anyhow::Result;
use oxipng::{optimize, InFile, Options, OutFile};

pub fn lossless_png(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let options = Options::max_compression(); // 设置压缩级别，范围是 0 到 6

    let input = PathBuf::from(input);
    let output = PathBuf::from(output);

    optimize(
        &InFile::Path(input),
        &OutFile::Path {
            path: Some(output),
            preserve_attrs: false, // 是否保留属性
        },
        &options,
    )?;

    // oxipng::optimize_from_memory(data, opts);

    Ok(())
}

// https://github.com/valterkraemer/imagequant-wasm/blob/main/src/lib.rs
pub async fn lossy_png(input: &str, output: &str) -> Result<()> {
    let image = lodepng::decode32_file(input)?;
    let rgba = image.buffer;
    let width = image.width;
    let height = image.height;

    let mut lib = imagequant::new();
    lib.set_speed(4)?;
    lib.set_quality(65, 80)?;
    // lib.set_max_colors(128)?;

    let mut img = lib.new_image(rgba, width, height, 0.0)?;

    let mut res = lib.quantize(&mut img)?;
    // res.set_dithering_level(1.0)?;

    let (palette, pixels) = res.remapped(&mut img)?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(palette.as_slice())?;
    encoder.encode_file(output, pixels.as_slice(), width, height)?;

    Ok(())
}
