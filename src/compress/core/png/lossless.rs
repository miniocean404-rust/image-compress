use oxipng::Deflaters::Libdeflater;
use oxipng::Options;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn to_file(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn to_mem(mem: &Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // let mut oxipng_options = oxipng::Options::default();
    // oxipng_options.deflate = Zopfli { iterations: NonZeroU8::new(15).ok_or("")?};
    let mut oxipng_options = Options::from_preset(6);
    oxipng_options.deflate = Libdeflater { compression: 6 };
    let png_vec = oxipng::optimize_from_memory(mem.as_slice(), &oxipng_options)?;

    Ok(png_vec)
}
