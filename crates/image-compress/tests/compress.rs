use std::fs;
use image_compress::{
    compress::ImageCompress,
    export::{
        AvifOptions, ImageQuantOptions, MozJpegOptions, OxiPngOptions, WebPOptions,
    },
};

#[test]
fn oxipng_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    let mut ins = ImageCompress::new(byte_vec).with_options(OxiPngOptions::default());
    let _ = ins.compress()?;

    println!("{}", ins);
    Ok(())
}

#[test]
fn image_quant_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/png/time-icon.png")?;

    let mut ins = ImageCompress::new(byte_vec).with_options(ImageQuantOptions::default());
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}

#[test]
fn mozjpeg_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/jpeg/eye.jpg")?;

    let mut ins = ImageCompress::new(byte_vec).with_options(MozJpegOptions::default());
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}

#[test]
fn webp_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/webp/time-icon.webp")?;

    let mut ins = ImageCompress::new(byte_vec).with_options(WebPOptions::default());
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}

#[test]
fn avif_compress() -> anyhow::Result<()> {
    let byte_vec = fs::read("../../assets/image/avif/f1t.avif")?;

    let mut ins = ImageCompress::new(byte_vec).with_options(AvifOptions::default());
    let _ = ins.compress()?;

    println!("{:#?}", ins);

    Ok(())
}
