use std::fs;

use image_compress::compress::{
    AvifOptions, ImageCompress, ImageQuantOptions, MozJpegOptions, OxiPngOptions, WebPOptions,
};

#[test]
fn oxipng_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    dbg!(byte_vec.len());

    let ins = ImageCompress::new(byte_vec, 80);

    let result = ins.with_options(OxiPngOptions::default()).compress()?;

    dbg!(result.len());

    Ok(())
}

#[test]
fn image_quant_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    dbg!(byte_vec.len());

    let result = ImageCompress::new(byte_vec, 80)
        .with_options(ImageQuantOptions::default())
        .compress()?;

    dbg!(result.len());

    Ok(())
}

#[test]
fn mozjpeg_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/jpeg/eye.jpg")?;

    dbg!(byte_vec.len());

    let result = ImageCompress::new(byte_vec, 80)
        .with_options(MozJpegOptions::default())
        .compress()?;

    dbg!(result.len());

    Ok(())
}

#[test]
fn webp_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/webp/time-icon.webp")?;

    dbg!(byte_vec.len());

    let result = ImageCompress::new(byte_vec, 80)
        .with_options(WebPOptions::default())
        .compress()?;

    dbg!(result.len());

    Ok(())
}

#[test]
fn avif_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/avif/f1t.avif")?;

    dbg!(byte_vec.len());

    let result = ImageCompress::new(byte_vec, 80)
        .with_options(AvifOptions::default())
        .compress()?;

    dbg!(result.len());

    Ok(())
}
