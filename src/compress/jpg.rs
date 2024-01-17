use anyhow::Result;
use tracing::info;

pub fn jepg_compress() -> Result<()> {
    let a = std::panic::catch_unwind(|| -> Result<Vec<u8>> {
        let d =
            mozjpeg::Decompress::with_markers(mozjpeg::ALL_MARKERS).from_path("image/eye.jpg")?;

        d.width(); // FYI
        d.height();
        d.color_space() == mozjpeg::ColorSpace::JCS_YCbCr;
        for marker in d.markers() { /* read metadata or color profiles */ }

        // rgb() enables conversion
        let mut image = d.rgb()?;
        image.width();
        image.height();
        image.color_space() == mozjpeg::ColorSpace::JCS_RGB;

        let pixels = image.read_scanlines()?;
        image.finish()?;
        info!(writer = ?pixels, "压缩图片");

        let comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

        let mut comp = comp.start_compress(Vec::new())?; // any io::Write will work

        comp.write_scanlines(&pixels[..])?;

        let writer = comp.finish()?;

        Ok(writer)
    });

    Ok(())
}
