use std::{
    io::{self, Cursor},
    mem,
    panic::AssertUnwindSafe,
};

use mozjpeg::qtable::*;
use zune_core::{
    bit_depth::BitDepth, bytestream::ZByteWriterTrait, colorspace::ColorSpace, log,
    options::DecoderOptions,
};
use zune_image::{codecs::ImageFormat, errors::ImageErrors, image::Image, traits::EncoderTrait};
use crate::codecs::jpeg::encoder::options::{MozJpegOptions, QtableOptimize};

/// A MozJpeg encoder
#[derive(Default, Debug)]
pub struct MozJpegEncoder {
    options: MozJpegOptions,
}

struct MozJpegTempVt<T: ZByteWriterTrait> {
    inner: T,
    bytes_written: usize,
}

impl<T: ZByteWriterTrait> io::Write for MozJpegTempVt<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.inner.write_bytes(buf).map_err(|e| match e {
            zune_core::bytestream::ZByteIoError::StdIoError(e) => e,
            e => io::Error::other(format!("{e:?}")),
        })?;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush_bytes().map_err(|e| match e {
            zune_core::bytestream::ZByteIoError::StdIoError(e) => e,
            e => io::Error::other(format!("{e:?}")),
        })
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.write_all_bytes(buf).map_err(|e| match e {
            zune_core::bytestream::ZByteIoError::StdIoError(e) => e,
            e => io::Error::other(format!("{e:?}")),
        })?;
        self.bytes_written += buf.len();
        Ok(())
    }
}

impl MozJpegEncoder {
    /// Create a new encoder
    pub fn new() -> Self {
        MozJpegEncoder::default()
    }

    /// Create a new encoder with specified options
    pub fn new_with_options(options: MozJpegOptions) -> Self {
        Self { options }
    }

    pub fn encode_mem(&mut self, buf: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let cursor = Cursor::new(buf);

        let image = Image::read(cursor, DecoderOptions::default())?;

        let mut compress_buf = Cursor::new(vec![]);
        self.encode(&image, &mut compress_buf)?;

        Ok(compress_buf.into_inner())
    }
}

impl EncoderTrait for MozJpegEncoder {
    fn name(&self) -> &'static str {
        "mozjpeg-encoder"
    }

    fn encode_inner<T: ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> Result<usize, ImageErrors> {
        let (width, height) = image.dimensions();
        let data = &image.flatten_to_u8()[0];

        std::panic::catch_unwind(AssertUnwindSafe(|| -> Result<usize, ImageErrors> {
            let format = match image.colorspace() {
                ColorSpace::RGB => mozjpeg::ColorSpace::JCS_RGB,
                ColorSpace::RGBA => mozjpeg::ColorSpace::JCS_EXT_RGBA,
                ColorSpace::YCbCr => mozjpeg::ColorSpace::JCS_YCbCr,
                ColorSpace::Luma => mozjpeg::ColorSpace::JCS_GRAYSCALE,
                ColorSpace::YCCK => mozjpeg::ColorSpace::JCS_YCCK,
                ColorSpace::CMYK => mozjpeg::ColorSpace::JCS_CMYK,
                ColorSpace::BGR => mozjpeg::ColorSpace::JCS_EXT_BGR,
                ColorSpace::BGRA => mozjpeg::ColorSpace::JCS_EXT_BGRA,
                ColorSpace::ARGB => mozjpeg::ColorSpace::JCS_EXT_ARGB,
                ColorSpace::Unknown => mozjpeg::ColorSpace::JCS_UNKNOWN,
                _ => mozjpeg::ColorSpace::JCS_UNKNOWN,
            };

            let mut comp = mozjpeg::Compress::new(format);

            comp.set_size(width, height);
            comp.set_quality(self.options.quality);

            if self.options.progressive {
                comp.set_progressive_mode();
            }

            comp.set_optimize_coding(self.options.optimize_coding);
            comp.set_smoothing_factor(self.options.smoothing);
            comp.set_color_space(match format {
                mozjpeg::ColorSpace::JCS_GRAYSCALE => {
                    log::warn!("Input colorspace is GRAYSCALE, using GRAYSCALE as output");

                    mozjpeg::ColorSpace::JCS_GRAYSCALE
                }
                mozjpeg::ColorSpace::JCS_CMYK => {
                    log::warn!("Input colorspace is CMYK, using CMYK as output");

                    mozjpeg::ColorSpace::JCS_CMYK
                }
                mozjpeg::ColorSpace::JCS_YCCK => {
                    log::warn!("Input colorspace is YCCK, using YCCK as output");

                    mozjpeg::ColorSpace::JCS_YCCK
                }

                _ => self.options.color_space,
            });
            comp.set_use_scans_in_trellis(self.options.trellis_multipass);

            if let Some(sb) = self.options.chroma_subsample {
                comp.set_chroma_sampling_pixel_sizes((sb, sb), (sb, sb))
            }

            let qtable = match self.options.qtable {
                Some(QtableOptimize::AhumadaWatsonPeterson) => {
                    Some(AhumadaWatsonPeterson.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::AnnexK_Luma) => {
                    Some(AnnexK_Luma.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::Flat) => {
                    Some(Flat.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::KleinSilversteinCarney) => {
                    Some(KleinSilversteinCarney.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::MSSSIM_Luma) => {
                    Some(MSSSIM_Luma.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::NRobidoux) => {
                    Some(NRobidoux.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::PSNRHVS_Luma) => {
                    Some(PSNRHVS_Luma.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::PetersonAhumadaWatson) => {
                    Some(PetersonAhumadaWatson.scaled(self.options.quality, self.options.quality))
                }
                Some(QtableOptimize::WatsonTaylorBorthwick) => {
                    Some(WatsonTaylorBorthwick.scaled(self.options.quality, self.options.quality))
                }
                None => None,
            };

            if let Some(qtable) = qtable {
                if self.options.luma {
                    comp.set_luma_qtable(&qtable);
                }

                if self.options.chroma {
                    comp.set_chroma_qtable(&qtable)
                }
            }

            let writer = MozJpegTempVt {
                inner: sink,
                bytes_written: 0,
            };

            let mut comp = comp.start_compress(writer)?;

            #[cfg(feature = "metadata")]
            {
                use exif::experimental::Writer;

                if let Some(metadata) = &image.metadata().exif() {
                    let mut writer = Writer::new();
                    // write first tags for exif
                    let mut buf = std::io::Cursor::new(b"Exif\x00\x00".to_vec());
                    // set buffer position to be bytes written, to ensure we don't overwrite anything
                    buf.set_position(6);

                    for metadatum in *metadata {
                        writer.push_field(metadatum);
                    }
                    let result = writer.write(&mut buf, false);
                    if result.is_ok() {
                        // add the exif tag to APP1 segment
                        comp.write_marker(mozjpeg::Marker::APP(1), buf.get_ref());
                    } else {
                        log::warn!("Writing exif failed {:?}", result);
                    }
                }
            }

            comp.write_scanlines(data)?;

            Ok(comp.finish()?.bytes_written)
        }))
        .map_err(|err| {
            if let Ok(mut err) = err.downcast::<String>() {
                ImageErrors::EncodeErrors(zune_image::errors::ImgEncodeErrors::Generic(mem::take(
                    &mut *err,
                )))
            } else {
                ImageErrors::EncodeErrors(zune_image::errors::ImgEncodeErrors::GenericStatic(
                    "Unknown error occurred during encoding",
                ))
            }
        })?
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[
            ColorSpace::Luma,
            ColorSpace::RGBA,
            ColorSpace::RGB,
            ColorSpace::YCCK,
            ColorSpace::CMYK,
            ColorSpace::BGR,
            ColorSpace::BGRA,
            ColorSpace::ARGB,
            ColorSpace::YCbCr,
        ]
    }

    fn format(&self) -> zune_image::codecs::ImageFormat {
        ImageFormat::JPEG
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight, BitDepth::Sixteen]
    }

    fn default_depth(&self, depth: BitDepth) -> BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}
