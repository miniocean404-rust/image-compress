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

    Ok(())
}
