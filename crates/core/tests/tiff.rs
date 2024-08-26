#![allow(unused_imports)]
mod utils;
use utils::mock::*;
use utils::path::*;

use image_compress_core::tiff::codec::decoder::TiffDecoder;
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::traits::EncoderTrait;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;

#[test]
fn decode() -> Result<(), Box<dyn std::error::Error>> {
    let byte_vec = fs::read(get_workspace_file_path("assets/image/tiff/f1t.tif"))?;
    let cursor = Cursor::new(&byte_vec);
    let reader = BufReader::new(cursor);

    let decoder = TiffDecoder::try_new(reader)?;

    let image = Image::from_decoder(decoder)?;

    assert_eq!(image.dimensions(), (48, 80));
    assert_eq!(image.colorspace(), ColorSpace::RGB);

    Ok(())
}
