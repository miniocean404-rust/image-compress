use oxipng::Deflaters::Libdeflater;
use oxipng::{Interlacing, Options};

#[cfg(feature = "filesystem")]
pub fn to_file(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

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

    let png_vec = to_mem(&in_file)?;

    // 写入文件
    let mut output_file = File::create(output)?;
    output_file.write_all(png_vec.as_slice())?;

    Ok(())
}

#[cfg(feature = "mem")]
pub fn mem(mem: &Vec<u8>, level: Option<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // let mut oxipng_options = oxipng::Options::default();
    // oxipng_options.deflate = Zopfli { iterations: NonZeroU8::new(15).ok_or("")?};

    let level = if let Some(l) = level { l } else { 6 };

    let mut oxipng_options = Options::from_preset(level);
    oxipng_options.deflate = Libdeflater { compression: 6 };
    oxipng_options.interlace = Some(Interlacing::None);

    let png_vec = oxipng::optimize_from_memory(mem.as_slice(), &oxipng_options)?;

    Ok(png_vec)
}
