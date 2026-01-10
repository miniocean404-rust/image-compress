use std::{
    ffi::CString,
    os::raw::{c_int, c_void},
    ptr,
};

/// GIF 写入标志常量 (来自 gifsicle gif.h)
/// 使用更小的最小代码大小
const GIF_WRITE_CAREFUL_MIN_CODE_SIZE: c_int = 1;
/// 更积极地清除 LZW 字典
const GIF_WRITE_EAGER_CLEAR: c_int = 2;
/// 启用 LZW 压缩优化
const GIF_WRITE_OPTIMIZE: c_int = 4;
/// 启用收缩优化（更激进的压缩）
const GIF_WRITE_SHRINK: c_int = 8;

use zune_core::{
    bit_depth::BitDepth,
    bytestream::{ZByteWriterTrait, ZWriter},
    colorspace::ColorSpace,
};
use zune_image::{
    codecs::ImageFormat,
    errors::{ImageErrors, ImgEncodeErrors},
    image::Image,
    traits::EncoderTrait,
};

use crate::codecs::gif::encoder::options::GifOptions;
use crate::error::{CompressError, Result};

/// GIF 编码器 (基于 gifsicle)
#[derive(Debug, Default)]
pub struct GifEncoder {
    options: GifOptions,
}

impl GifEncoder {
    /// 创建一个新的 GIF 编码器
    pub fn new() -> Self {
        GifEncoder::default()
    }

    /// 使用指定选项创建编码器
    pub fn new_with_options(options: GifOptions) -> Self {
        Self { options }
    }

    /// 从内存中的 GIF 数据进行压缩
    ///
    /// 注意：gifsicle 是基于文件的压缩库，需要通过临时文件进行处理
    pub fn encode_mem(&mut self, buf: &[u8]) -> Result<Vec<u8>> {
        // 使用 gifsicle 进行压缩
        self.compress_with_gifsicle(buf)
    }

    /// 使用 gifsicle 进行 GIF 压缩
    fn compress_with_gifsicle(&self, input_data: &[u8]) -> Result<Vec<u8>> {
        use std::io::Write;

        // 创建临时文件
        let temp_dir = std::env::temp_dir();
        let input_path = temp_dir.join(format!("gifsicle_input_{}.gif", std::process::id()));
        let output_path = temp_dir.join(format!("gifsicle_output_{}.gif", std::process::id()));

        // 写入输入文件
        let mut input_file = std::fs::File::create(&input_path)?;
        input_file.write_all(input_data)?;
        input_file.flush()?;
        drop(input_file);

        // 使用 gifsicle FFI 进行压缩
        let input_str = input_path
            .to_str()
            .ok_or_else(|| CompressError::InvalidData("输入路径包含无效字符".to_string()))?;

        let output_str = output_path
            .to_str()
            .ok_or_else(|| CompressError::InvalidData("输出路径包含无效字符".to_string()))?;

        let result = unsafe { self.compress_gif_file(input_str, output_str) };

        // 清理输入文件
        let _ = std::fs::remove_file(&input_path);

        if let Err(e) = result {
            let _ = std::fs::remove_file(&output_path);
            return Err(e);
        }

        // 读取输出文件
        let output_data = std::fs::read(&output_path)?;

        // 清理输出文件
        let _ = std::fs::remove_file(&output_path);

        Ok(output_data)
    }

    /// 使用 gifsicle FFI 压缩 GIF 文件
    unsafe fn compress_gif_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let input_cstr = CString::new(input_path)?;
        let output_cstr = CString::new(output_path)?;
        let read_mode = CString::new("rb")?;
        let write_mode = CString::new("wb")?;

        // 打开输入文件
        let input_file = libc::fopen(input_cstr.as_ptr(), read_mode.as_ptr());
        if input_file.is_null() {
            return Err(CompressError::GifEncode(format!(
                "无法打开输入文件: {}",
                input_path
            )));
        }

        // 读取 GIF 流
        let input_stream = gifsicle::Gif_ReadFile(input_file);
        libc::fclose(input_file);

        if input_stream.is_null() {
            return Err(CompressError::GifDecode("无法读取 GIF 文件".to_string()));
        }

        // 打开输出文件
        let output_file = libc::fopen(output_cstr.as_ptr(), write_mode.as_ptr());
        if output_file.is_null() {
            gifsicle::Gif_DeleteStream(input_stream);
            return Err(CompressError::GifEncode(format!(
                "无法创建输出文件: {}",
                output_path
            )));
        }

        // 设置压缩参数
        let padding: [*mut c_void; 7] = [ptr::null_mut(); 7];
        let loss = self.options.lossy as c_int;

        // 根据优化级别设置标志
        // optimize_level 1: 基本优化 (OPTIMIZE + CAREFUL_MIN_CODE_SIZE)
        // optimize_level 2: 中等优化 (OPTIMIZE + EAGER_CLEAR)
        // optimize_level 3: 最大优化 (OPTIMIZE + SHRINK + EAGER_CLEAR)
        // 所有级别都启用 OPTIMIZE 以确保基本的帧优化
        let flags = match self.options.optimize_level {
            1 => GIF_WRITE_OPTIMIZE | GIF_WRITE_CAREFUL_MIN_CODE_SIZE,
            2 => GIF_WRITE_OPTIMIZE | GIF_WRITE_EAGER_CLEAR,
            _ => GIF_WRITE_OPTIMIZE | GIF_WRITE_SHRINK | GIF_WRITE_EAGER_CLEAR,
        };

        let gc_info = gifsicle::Gif_CompressInfo {
            flags,
            loss,
            padding,
        };

        // 写入压缩后的 GIF
        let write_result = gifsicle::Gif_FullWriteFile(input_stream, &gc_info, output_file);
        libc::fclose(output_file);
        gifsicle::Gif_DeleteStream(input_stream);

        match write_result {
            1 => Ok(()),
            _ => Err(CompressError::GifEncode("GIF 压缩失败".to_string())),
        }
    }
}

impl EncoderTrait for GifEncoder {
    fn name(&self) -> &'static str {
        "gif-encoder"
    }

    fn encode_inner<T: ZByteWriterTrait>(
        &mut self,
        image: &Image,
        sink: T,
    ) -> std::result::Result<usize, ImageErrors> {
        let (width, height) = image.dimensions();
        let mut writer = ZWriter::new(sink);

        // 首先将图像编码为标准 GIF 格式
        let gif_data = self.encode_to_gif(image, width, height)?;

        // 然后使用 gifsicle 进行压缩
        let compressed_data = self
            .compress_with_gifsicle(&gif_data)
            .map_err(|e| ImgEncodeErrors::ImageEncodeErrors(e.to_string()))?;

        writer.write(&compressed_data).map_err(|e| {
            ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(format!("{e:?}")))
        })?;

        Ok(writer.bytes_written())
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGB, ColorSpace::RGBA]
    }

    fn format(&self) -> ImageFormat {
        // zune_image 没有 GIF 格式，使用 Unknown
        ImageFormat::Unknown
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight]
    }

    fn default_depth(&self, _depth: BitDepth) -> BitDepth {
        BitDepth::Eight
    }

    fn supports_animated_images(&self) -> bool {
        true
    }
}

impl GifEncoder {
    /// 将图像编码为标准 GIF 格式
    fn encode_to_gif(
        &self,
        image: &Image,
        width: usize,
        height: usize,
    ) -> std::result::Result<Vec<u8>, ImageErrors> {
        let mut buffer = Vec::new();

        {
            let mut encoder = gif::Encoder::new(&mut buffer, width as u16, height as u16, &[])
                .map_err(|e| {
                    ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string()))
                })?;

            // 设置重复次数 (0 表示无限循环)
            encoder.set_repeat(gif::Repeat::Infinite).map_err(|e| {
                ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string()))
            })?;

            let frames = image.flatten_to_u8();
            let colorspace = image.colorspace();

            for frame_data in frames.iter() {
                // 转换为 RGBA 格式
                let rgba_data = match colorspace {
                    ColorSpace::RGBA => frame_data.clone(),
                    ColorSpace::RGB => {
                        // RGB -> RGBA
                        let mut rgba = Vec::with_capacity(frame_data.len() / 3 * 4);
                        for chunk in frame_data.chunks(3) {
                            rgba.extend_from_slice(chunk);
                            rgba.push(255);
                        }
                        rgba
                    }
                    _ => {
                        return Err(ImageErrors::EncodeErrors(
                            ImgEncodeErrors::UnsupportedColorspace(
                                colorspace,
                                self.supported_colorspaces(),
                            ),
                        ))
                    }
                };

                // 获取帧延迟时间
                let delay = if image.is_animated() {
                    // 从 zune_image 获取帧延迟 (假设以毫秒为单位)
                    // GIF 延迟单位是 10ms
                    5 // 默认 50ms
                } else {
                    0
                };

                let mut frame = gif::Frame::from_rgba_speed(
                    width as u16,
                    height as u16,
                    &mut rgba_data.clone(),
                    10, // 速度参数 (1-30, 10 是默认值)
                );
                frame.delay = delay;

                encoder.write_frame(&frame).map_err(|e| {
                    ImageErrors::EncodeErrors(ImgEncodeErrors::ImageEncodeErrors(e.to_string()))
                })?;
            }
        }

        Ok(buffer)
    }
}
