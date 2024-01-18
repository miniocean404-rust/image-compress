use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use anyhow::{Ok, Result};

pub fn lossless_jpeg(input: &str, output: &str) -> Result<()> {
    // let _ = std::panic::catch_unwind(|| -> Result<()> { Ok(()) });

    let file = File::open(input)?;
    let buf_reader = BufReader::new(file);

    let file = File::create(output)?;
    let writer = BufWriter::new(file);

    let decode = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_reader(buf_reader)?;

    // for marker in d.markers() {}

    let mut image = decode.rgb()?;

    let pixels = image.read_scanlines::<u8>()?;

    let width = image.width();
    let height = image.height();

    image.finish()?;

    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    // 必须设置宽高
    comp.set_size(width, height);
    comp.set_scan_optimization_mode(mozjpeg::ScanMode::AllComponentsTogether);
    comp.set_quality(60.0);
    // comp.set_smoothing_factor(1); // 消除噪点，减少大小

    let mut comp = comp.start_compress(writer)?;

    comp.write_scanlines(&pixels)?;

    comp.finish()?;

    // let read_metadata = fs::metadata(input)?;
    // let out_metadata = fs::metadata(output)?;
    // info!(
    //     "metadata: {:?} {:?}",
    //     out_metadata.len(),
    //     read_metadata.len()
    // );

    Ok(())
}
