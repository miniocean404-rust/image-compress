use image_compress::{
    compress::{ImageCompress, Options},
    export::{ImageQuantOptions, OxiPngOptions},
};

#[cfg(feature = "native")]
use image_compress::export::{AvifOptions, MozJpegOptions, WebPOptions};

use std::fs;

#[test]
fn oxipng_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    let mut ins = ImageCompress::new()
        .with_buffer(byte_vec)
        .with_options(Options::OxiPng(OxiPngOptions::default()));
    let _ = ins.compress()?;

    Ok(())
}

#[test]
fn image_quant_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    let mut ins = ImageCompress::new()
        .with_buffer(byte_vec)
        .with_options(Options::ImageQuant(ImageQuantOptions::default()));
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}

#[test]
#[cfg(feature = "native")]
fn mozjpeg_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/jpeg/eye.jpg")?;

    let mut ins = ImageCompress::new()
        .with_buffer(byte_vec)
        .with_options(Options::MozJpeg(MozJpegOptions::default()));
    let _ = ins.compress()?;

    Ok(())
}

#[test]
#[cfg(feature = "native")]
fn webp_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/webp/time-icon.webp")?;

    let mut ins = ImageCompress::new()
        .with_buffer(byte_vec)
        .with_options(Options::WebP(WebPOptions::default()));
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}

#[test]
#[cfg(feature = "native")]
fn avif_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/avif/f1t.avif")?;

    let mut ins = ImageCompress::new()
        .with_buffer(byte_vec)
        .with_options(Options::Avif(AvifOptions::default()));
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}
