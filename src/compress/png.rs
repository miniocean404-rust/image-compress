use std::{error::Error, path::PathBuf, vec};

use anyhow::Result;
use image::{GenericImageView, Rgba};
use oxipng::{optimize, InFile, Options, OutFile};
use rgb::FromSlice;
use tracing::{error, info};

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

// https://github.com/ImageOptim/libimagequant/blob/main/examples/basic.rs
pub fn lossy_png(input: &str, output: &str) -> Result<()> {
    let img = image::open(input)?;
    let binding = img.to_rgba8();
    let rgba_v8 = binding.as_rgba();
    let (width, height) = img.dimensions();

    let mut lib = imagequant::new();
    lib.set_speed(4)?;
    lib.set_quality(65, 80)?;

    let mut img = lib.new_image(
        rgba_v8,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        0.0,
    )?;

    let mut res = lib.quantize(&mut img)?;

    res.set_dithering_level(1.0)?;

    let (palette, pixels) = res.remapped(&mut img)?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(palette.as_slice())?;
    encoder.encode_file(
        output,
        pixels.as_slice(),
        width.try_into()?,
        height.try_into()?,
    )?;

    Ok(())
}
