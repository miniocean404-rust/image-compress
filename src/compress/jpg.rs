use std::io::Write;
use std::{
    fs::File,
    io::{self, BufWriter},
};

use anyhow::{Ok, Result};
use tracing::info;

pub fn jeg_compress() -> Result<()> {
    let a = std::panic::catch_unwind(|| -> Result<()> { Ok(()) });

    let decode =
        mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_path("image/jpg/eye.jpg")?;

    decode.color_space() == mozjpeg::ColorSpace::JCS_YCbCr;
    // for marker in d.markers() {}

    // rgb() enables conversion
    let mut image = decode.rgb()?;

    image.color_space() == mozjpeg::ColorSpace::JCS_RGB;

    let pixels = image.read_scanlines::<u8>()?;

    image.finish()?;

    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    comp.set_quality(80.0);

    let file = File::create("output.jpeg")?;
    let writer = BufWriter::new(file);

    let mut comp = comp.start_compress(writer)?;

    comp.write_scanlines(&pixels)?;

    let writer = comp.finish()?;

    Ok(())
}
