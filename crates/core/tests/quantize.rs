mod utils;

use utils::mock::*;

use image_compress_core::operations::quantize::Quantize;
use zune_core::colorspace::ColorSpace;
use zune_image::traits::OperationsTrait;

#[test]
fn quantize_u8() {
    let quantize = Quantize::new(75, None);
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    dbg!(&result);

    assert!(result.is_ok());
}

#[test]
fn dither_u8() {
    let quantize = Quantize::new(75, Some(0.75));
    let mut image = create_test_image_u8(200, 200, ColorSpace::RGBA);

    let result = quantize.execute(&mut image);

    dbg!(&result);

    assert!(result.is_ok());
}
