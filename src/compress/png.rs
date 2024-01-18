use std::{error::Error, path::PathBuf};

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

// https://github.com/ImageOptim/libimagequant/blob/main/examples/basic.rs
pub fn lossy_png(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut lib = imagequant::new();
    lib.set_speed(4)?;
    lib.set_quality(65, 80)?;

    // let mut img = lib.new_image(pixels, width, height, 0);

    // let mut res = match lib.quantize(&mut img) {
    //     Ok(res) => res,
    //     Err(err) => panic!("量化失败, 因为: {err:?}"),
    // };

    // res.set_dithering_level(1.0)?;

    Ok(())
}
