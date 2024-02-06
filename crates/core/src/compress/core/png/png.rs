use std::borrow::BorrowMut;

use rgb::{
    alt::{GRAY8, GRAYA8},
    AsPixels, FromSlice, RGB8, RGBA8,
};
use tracing::instrument::WithSubscriber;

pub fn encode(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut buf = Vec::new();

    {
        let mut encoder = png::Encoder::new(&mut buf, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(data).unwrap();
    }

    buf
}

// Convert pixels in-place within buffer containing source data but preallocated
// for entire [num_pixels * sizeof(RGBA)].
// This works because all the color types are <= RGBA by size.
fn expand_pixels<Src: Copy>(buf: &mut [u8], to_rgba: impl Fn(Src) -> RGBA8)
where
    [u8]: AsPixels<Src> + FromSlice<u8>,
{
    assert!(std::mem::size_of::<Src>() <= std::mem::size_of::<RGBA8>());
    let num_pixels = buf.len() / 4;
    for i in (0..num_pixels).rev() {
        let src_pixel = buf.as_pixels()[i];
        buf.as_rgba_mut()[i] = to_rgba(src_pixel);
    }
}

pub fn decode(mut data: &[u8]) -> Vec<u8> {
    let mut decoder = png::Decoder::new(&mut data);
    decoder.set_transformations(
        png::Transformations::EXPAND | // Turn images <8bit to 8bit
        png::Transformations::STRIP_16, // Turn 16bit into 8 bit
    );

    let mut reader = decoder.read_info().expect("期望读取图片信息");
    let mut buf = vec![0; reader.output_buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    let info = reader.info();

    // Transformations::EXPAND will expand indexed palettes and lower-bit
    // grayscales to higher color types, but we still need to transform
    // the rest to RGBA.
    match info.color_type {
        png::ColorType::Rgba => {}
        png::ColorType::Rgb => expand_pixels(&mut buf, RGB8::into),
        png::ColorType::GrayscaleAlpha => expand_pixels(&mut buf, GRAYA8::into),
        png::ColorType::Grayscale => expand_pixels(&mut buf, |gray: GRAY8| GRAYA8::from(gray).into()),
        png::ColorType::Indexed => {
            unreachable!("找到已索引的颜色类型，但期望它已经展开")
        }
    }

    buf
}
