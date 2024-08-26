use std::marker::PhantomData;

use webp::{AnimDecoder, DecodeAnimImage};
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{errors::ImageErrors, frame::Frame, image::Image, traits::DecoderTrait};

/// A WebP decoder
pub struct WebPDecoder {
    inner: DecodeAnimImage,
    phantom: PhantomData<Vec<u8>>,
}

impl WebPDecoder {
    /// Create a new webp decoder that reads data from `source`
    pub fn try_new(buf: &[u8]) -> Result<WebPDecoder, ImageErrors> {
        let decoder = AnimDecoder::new(buf);
        let img = decoder.decode().map_err(ImageErrors::ImageDecodeErrors)?;

        Ok(WebPDecoder {
            inner: img,
            phantom: PhantomData,
        })
    }
}

impl DecoderTrait for WebPDecoder {
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let (width, height) = <WebPDecoder as DecoderTrait>::dimensions(self).unwrap();
        let color = <WebPDecoder as DecoderTrait>::out_colorspace(self);

        let frames = self
            .inner
            .into_iter()
            .enumerate()
            .map(|(idx, frame)| Frame::from_u8(frame.get_image(), color, idx, frame.get_time_ms() as usize))
            .collect::<Vec<_>>();

        Ok(Image::new_frames(frames, BitDepth::Eight, width, height, color))
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        let frame = self.inner.get_frame(0).unwrap();

        Some((frame.width() as usize, frame.height() as usize))
    }

    fn out_colorspace(&self) -> ColorSpace {
        let frame = self.inner.get_frame(0).unwrap();

        match frame.get_layout() {
            webp::PixelLayout::Rgb => ColorSpace::RGB,
            webp::PixelLayout::Rgba => ColorSpace::RGBA,
        }
    }

    fn name(&self) -> &'static str {
        "webp"
    }
}
