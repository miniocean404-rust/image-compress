use std::fs::File;
use std::io::Write;
use std::{error::Error, fs};

use anyhow::Result;
use oxipng::Deflaters::Libdeflater;
use oxipng::Options;

pub fn lossless_png(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    // let options = Options::max_compression(); // 设置压缩级别，范围是 0 到 6

    // let input = PathBuf::from(input);
    // let output = PathBuf::from(output);
    //
    // optimize(
    //     &InFile::Path(input),
    //     &OutFile::Path {
    //         path: Some(output),
    //         preserve_attrs: false, // 是否保留属性
    //     },
    //     &options,
    // )?;

    let in_file = fs::read(input)?;

    // let mut oxipng_options = oxipng::Options::default();
    // oxipng_options.deflate = Zopfli { iterations: NonZeroU8::new(15).ok_or("")?};
    let mut oxipng_options = Options::from_preset(6);
    oxipng_options.deflate = Libdeflater { compression: 6 };
    let png_vec = oxipng::optimize_from_memory(in_file.as_slice(), &oxipng_options)?;

    // 写入文件
    let mut output_file = File::create(output)?;
    output_file.write_all(png_vec.as_slice())?;

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
    lib.set_quality(0, 80)?;
    // lib.set_max_colors(128)?;

    let mut img = lib.new_image(rgba, width, height, 0.0)?;

    let mut res = lib.quantize(&mut img)?;
    // res.set_dithering_level(1.0)?;

    let (palette, pixels) = res.remapped(&mut img)?;

    let mut encoder = lodepng::Encoder::new();
    encoder.set_palette(palette.as_slice())?;

    // 写入文件
    // encoder.encode_file(output, pixels.as_slice(), width, height)?;
    let png_vec = encoder.encode(pixels.as_slice(), width, height)?;
    let mut output_file = File::create(output)?;
    output_file.write_all(png_vec.as_slice())?;

    Ok(())
}
