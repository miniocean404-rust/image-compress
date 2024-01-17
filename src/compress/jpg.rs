use std::{fs::File, io::BufWriter};

use anyhow::{Ok, Result};
use tracing::info;

pub fn jeg_compress() -> Result<()> {
    let _ = std::panic::catch_unwind(|| -> Result<()> {
        let decode = mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS)
            .from_path("image/jpg/eye.jpg")?;

        // for marker in d.markers() {}

        // rgb() enables conversion
        let mut image = decode.rgb()?;

        let pixels = image.read_scanlines::<u8>()?;

        let width = image.width();
        let height = image.height();

        // image.finish()?;

        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
        comp.set_scan_optimization_mode(mozjpeg::ScanMode::AllComponentsTogether);

        // 必须设置宽高
        comp.set_size(width, height);
        comp.set_quality(60.0);

        let file = File::create("dist/output.jpg")?;
        let writer = BufWriter::new(file);

        let mut comp = comp.start_compress(writer)?;

        comp.write_scanlines(&pixels)?;

        comp.finish()?;

        Ok(())
    });

    Ok(())
}
