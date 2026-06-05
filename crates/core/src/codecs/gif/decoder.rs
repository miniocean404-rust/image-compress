use std::io::BufRead;

use gif::DecodeOptions;
use zune_core::{bit_depth::BitDepth, colorspace::ColorSpace};
use zune_image::{errors::ImageErrors, frame::Frame, image::Image, traits::DecoderTrait};

/// GIF 解码器
pub struct GifDecoder<R: BufRead> {
    reader: gif::Decoder<R>,
    width: u16,
    height: u16,
    #[allow(dead_code)]
    global_palette: Option<Vec<u8>>,
}

impl<R: BufRead> GifDecoder<R> {
    /// 创建一个新的 GIF 解码器
    pub fn try_new(source: R) -> Result<GifDecoder<R>, ImageErrors> {
        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let decoder = options
            .read_info(source)
            .map_err(|e| ImageErrors::ImageDecodeErrors(e.to_string()))?;

        let width = decoder.width();
        let height = decoder.height();
        let global_palette = decoder.global_palette().map(|p| p.to_vec());

        Ok(GifDecoder {
            reader: decoder,
            width,
            height,
            global_palette,
        })
    }
}

impl<R: BufRead> DecoderTrait for GifDecoder<R> {
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let (width, height) = self
            .dimensions()
            .ok_or_else(|| ImageErrors::ImageDecodeErrors("无法获取 GIF 图像尺寸".to_string()))?;
        let colorspace = ColorSpace::RGBA;

        let mut frames = Vec::new();
        let mut frame_idx = 0;

        while let Some(frame) = self
            .reader
            .read_next_frame()
            .map_err(|e| ImageErrors::ImageDecodeErrors(e.to_string()))?
        {
            // 获取帧延迟时间 (单位: 10ms)
            let delay_ms = frame.delay as usize * 10;

            // 创建完整尺寸的缓冲区
            let mut full_buffer = vec![0u8; width * height * 4];

            // 获取帧的位置和尺寸
            let frame_left = frame.left as usize;
            let frame_top = frame.top as usize;
            let frame_width = frame.width as usize;
            let frame_height = frame.height as usize;

            // 将帧数据复制到正确的位置
            for y in 0..frame_height {
                for x in 0..frame_width {
                    let src_idx = (y * frame_width + x) * 4;
                    let dst_x = frame_left + x;
                    let dst_y = frame_top + y;

                    if dst_x < width && dst_y < height {
                        let dst_idx = (dst_y * width + dst_x) * 4;
                        if src_idx + 4 <= frame.buffer.len() && dst_idx + 4 <= full_buffer.len() {
                            full_buffer[dst_idx..dst_idx + 4].copy_from_slice(&frame.buffer[src_idx..src_idx + 4]);
                        }
                    }
                }
            }

            let zune_frame = Frame::from_u8(&full_buffer, colorspace, frame_idx, delay_ms);
            frames.push(zune_frame);
            frame_idx += 1;
        }

        if frames.is_empty() {
            return Err(ImageErrors::ImageDecodeErrors("GIF 文件中没有帧".to_string()));
        }

        Ok(Image::new_frames(frames, BitDepth::Eight, width, height, colorspace))
    }

    fn dimensions(&self) -> Option<(usize, usize)> {
        Some((self.width as usize, self.height as usize))
    }

    fn out_colorspace(&self) -> ColorSpace {
        ColorSpace::RGBA
    }

    fn name(&self) -> &'static str {
        "gif"
    }
}
