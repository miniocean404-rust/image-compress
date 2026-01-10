use std::{
    io::{self, Cursor},
    mem,
    panic::AssertUnwindSafe,
};

use crate::codecs::jpeg::encoder::options::{MozJpegOptions, QtableOptimize, QtableOptimizeChroma};
use crate::error::{CompressError, Result};
use mozjpeg::qtable::*;
use zune_core::{
    bit_depth::BitDepth, bytestream::ZByteWriterTrait, colorspace::ColorSpace, log,
    options::DecoderOptions,
};
use zune_image::{codecs::ImageFormat, errors::ImageErrors, image::Image, traits::EncoderTrait};

// ============================================================================
// 写入适配器
// ============================================================================

/// ZByteWriterTrait 到 std::io::Write 的适配器
///
/// MozJpeg 需要 std::io::Write，而 zune_image 使用 ZByteWriterTrait，
/// 此结构体用于桥接两者，同时跟踪写入的字节数。
struct WriteAdapter<T: ZByteWriterTrait> {
    inner: T,
    bytes_written: usize,
}

impl<T: ZByteWriterTrait> WriteAdapter<T> {
    /// 将 ZByteIoError 转换为 std::io::Error
    fn convert_error(e: zune_core::bytestream::ZByteIoError) -> io::Error {
        match e {
            zune_core::bytestream::ZByteIoError::StdIoError(e) => e,
            e => io::Error::other(format!("{e:?}")),
        }
    }
}

impl<T: ZByteWriterTrait> io::Write for WriteAdapter<T> {
    /// 写入数据并跟踪字节数
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.inner.write_bytes(buf).map_err(Self::convert_error)?;
        self.bytes_written += bytes_written;
        Ok(bytes_written)
    }

    /// 刷新底层写入器
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush_bytes().map_err(Self::convert_error)
    }

    /// 写入全部数据并跟踪字节数
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner
            .write_all_bytes(buf)
            .map_err(Self::convert_error)?;
        self.bytes_written += buf.len();
        Ok(())
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 将 zune_core 色彩空间映射到 MozJpeg 色彩空间
fn map_colorspace(cs: ColorSpace) -> mozjpeg::ColorSpace {
    match cs {
        ColorSpace::RGB => mozjpeg::ColorSpace::JCS_RGB,
        ColorSpace::RGBA => mozjpeg::ColorSpace::JCS_EXT_RGBA,
        ColorSpace::YCbCr => mozjpeg::ColorSpace::JCS_YCbCr,
        ColorSpace::Luma => mozjpeg::ColorSpace::JCS_GRAYSCALE,
        ColorSpace::YCCK => mozjpeg::ColorSpace::JCS_YCCK,
        ColorSpace::CMYK => mozjpeg::ColorSpace::JCS_CMYK,
        ColorSpace::BGR => mozjpeg::ColorSpace::JCS_EXT_BGR,
        ColorSpace::BGRA => mozjpeg::ColorSpace::JCS_EXT_BGRA,
        ColorSpace::ARGB => mozjpeg::ColorSpace::JCS_EXT_ARGB,
        _ => mozjpeg::ColorSpace::JCS_UNKNOWN,
    }
}

/// 根据输入色彩空间确定输出色彩空间
///
/// 某些色彩空间（灰度、CMYK、YCCK）必须保持原样输出，
/// 其他情况使用用户配置的色彩空间。
fn determine_output_colorspace(
    input: mozjpeg::ColorSpace,
    configured: mozjpeg::ColorSpace,
) -> mozjpeg::ColorSpace {
    match input {
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
        _ => configured,
    }
}

/// 根据量化表类型和质量生成缩放后的量化表
///
/// 量化表影响 JPEG 压缩的质量和文件大小，不同的量化表算法
/// 针对不同的视觉质量指标进行了优化。
fn build_qtable(
    qtable_type: Option<&QtableOptimize>,
    quality: f32,
) -> Option<mozjpeg::qtable::QTable> {
    qtable_type.map(|qt| match qt {
        QtableOptimize::AhumadaWatsonPeterson => AhumadaWatsonPeterson.scaled(quality, quality),
        QtableOptimize::AnnexK_Luma => AnnexK_Luma.scaled(quality, quality),
        QtableOptimize::Flat => Flat.scaled(quality, quality),
        QtableOptimize::KleinSilversteinCarney => KleinSilversteinCarney.scaled(quality, quality),
        QtableOptimize::MSSSIM_Luma => MSSSIM_Luma.scaled(quality, quality),
        QtableOptimize::NRobidoux => NRobidoux.scaled(quality, quality),
        QtableOptimize::PSNRHVS_Luma => PSNRHVS_Luma.scaled(quality, quality),
        QtableOptimize::PetersonAhumadaWatson => PetersonAhumadaWatson.scaled(quality, quality),
        QtableOptimize::WatsonTaylorBorthwick => WatsonTaylorBorthwick.scaled(quality, quality),
    })
}

/// 根据色度量化表类型和质量生成缩放后的色度量化表
fn build_chroma_qtable(
    qtable_type: Option<&QtableOptimizeChroma>,
    quality: f32,
) -> Option<mozjpeg::qtable::QTable> {
    qtable_type.map(|qt| match qt {
        QtableOptimizeChroma::AnnexK_Chroma => AnnexK_Chroma.scaled(quality, quality),
        QtableOptimizeChroma::MSSSIM_Chroma => MSSSIM_Chroma.scaled(quality, quality),
        QtableOptimizeChroma::PSNRHVS_Chroma => PSNRHVS_Chroma.scaled(quality, quality),
    })
}

/// 根据质量自动计算色度子采样参数
///
/// 高质量(>90)使用 4:4:4 无子采样，保留最多细节
/// 中等质量(>70)使用 4:2:2，平衡质量和压缩
/// 低质量(<=70)使用 4:2:0，最大压缩
fn auto_chroma_subsample(quality: f32) -> (u8, u8) {
    match quality {
        q if q > 90.0 => (1, 1), // 4:4:4 无子采样
        q if q > 70.0 => (2, 1), // 4:2:2
        _ => (2, 2),             // 4:2:0 最大压缩
    }
}

// ============================================================================
// MozJpeg 编码器
// ============================================================================

/// MozJpeg JPEG 编码器
///
/// 使用 Mozilla 的 MozJpeg 库进行高效 JPEG 压缩。
/// 支持渐进式编码、多种量化表优化和色度子采样配置。
///
/// # 示例
/// ```ignore
/// let encoder = MozJpegEncoder::new_with_options(MozJpegOptions {
///     quality: 80.0,
///     progressive: true,
///     ..Default::default()
/// });
/// let compressed = encoder.encode_mem(&image_data)?;
/// ```
#[derive(Default, Debug)]
pub struct MozJpegEncoder {
    options: MozJpegOptions,
}

impl MozJpegEncoder {
    /// 创建使用默认选项的新编码器
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建使用指定选项的新编码器
    pub fn new_with_options(options: MozJpegOptions) -> Self {
        Self { options }
    }

    /// 从内存缓冲区编码图像
    ///
    /// # 参数
    /// * `buf` - 输入图像数据（支持多种格式，由 zune_image 自动检测）
    ///
    /// # 返回
    /// 压缩后的 JPEG 数据
    pub fn encode_mem(&mut self, buf: &[u8]) -> Result<Vec<u8>> {
        let cursor = Cursor::new(buf);
        let image = Image::read(cursor, DecoderOptions::default())
            .map_err(|e| CompressError::JpegDecode(e.to_string()))?;

        let mut compress_buf = Cursor::new(vec![]);
        // 使用 MozJpegEncoder 进行编码
        self.encode(&image, &mut compress_buf)
            .map_err(|e| CompressError::JpegEncode(e.to_string()))?;

        Ok(compress_buf.into_inner())
    }

    /// 配置压缩器的基本参数
    fn configure_compressor(
        &self,
        comp: &mut mozjpeg::Compress,
        width: usize,
        height: usize,
        input_colorspace: mozjpeg::ColorSpace,
    ) {
        // 设置图像尺寸和质量
        comp.set_size(width, height);
        comp.set_quality(self.options.quality);

        // 渐进式编码：生成可逐步显示的 JPEG
        if self.options.progressive {
            comp.set_progressive_mode();
        }

        // 优化霍夫曼编码表
        comp.set_optimize_coding(self.options.optimize_coding);
        // 平滑因子：减少块效应
        comp.set_smoothing_factor(self.options.smoothing);
        // 设置输出色彩空间
        comp.set_color_space(determine_output_colorspace(
            input_colorspace,
            self.options.color_space,
        ));

        // Trellis 量化配置
        // Trellis 多遍优化：在多次扫描中优化量化
        comp.set_use_scans_in_trellis(self.options.trellis_multipass);

        // 渐进式扫描优化：使渐进式图像文件更小
        comp.set_optimize_scans(self.options.optimize_scans);

        // TODO: mozjpeg crate 0.10.13 未暴露以下方法，需要等待更新或使用 mozjpeg-sys
        // 这些选项已在 MozJpegOptions 中定义，但当前无法生效
        // - set_trellis_quant (DC 系数 Trellis 量化)
        // - set_trellis_quant_ac (AC 系数 Trellis 量化)
        // - set_overshoot_deringing (减少振铃效应)
        // 底层 mozjpeg-sys 支持这些功能：
        // - JBOOLEAN_TRELLIS_QUANT
        // - JBOOLEAN_TRELLIS_QUANT_DC
        // - JBOOLEAN_OVERSHOOT_DERINGING

        // 色度子采样配置
        if let Some(sb) = self.options.chroma_subsample {
            // 用户显式指定的子采样
            comp.set_chroma_sampling_pixel_sizes((sb, sb), (sb, sb));
        } else if self.options.auto_chroma_subsample {
            // 根据质量自动选择子采样
            let (h, v) = auto_chroma_subsample(self.options.quality);
            comp.set_chroma_sampling_pixel_sizes((h, v), (h, v));
        }
    }

    /// 应用量化表配置
    fn apply_qtable(&self, comp: &mut mozjpeg::Compress) {
        // 如果用户显式指定了量化表，优先使用用户配置
        if self.options.qtable.is_some() {
            // 亮度量化表
            if let Some(qtable) = build_qtable(self.options.qtable.as_ref(), self.options.quality) {
                if self.options.luma {
                    comp.set_luma_qtable(&qtable);
                }
                // 如果没有指定色度专用量化表，且启用了 chroma，则使用亮度量化表
                if self.options.chroma && self.options.qtable_chroma.is_none() {
                    comp.set_chroma_qtable(&qtable);
                }
            }

            // 色度专用量化表（优先级高于通用量化表）
            if self.options.chroma {
                if let Some(chroma_qtable) =
                    build_chroma_qtable(self.options.qtable_chroma.as_ref(), self.options.quality)
                {
                    comp.set_chroma_qtable(&chroma_qtable);
                }
            }
        } else if self.options.auto_qtable && self.options.quality >= 80.0 {
            // 自动量化表选择：高质量模式使用 MSSSIM 优化量化表
            // MSSSIM 量化表针对人眼感知优化，在高质量场景下能获得更好的视觉效果
            let luma_qtable =
                build_qtable(Some(&QtableOptimize::MSSSIM_Luma), self.options.quality);
            let chroma_qtable = build_chroma_qtable(
                Some(&QtableOptimizeChroma::MSSSIM_Chroma),
                self.options.quality,
            );

            if let Some(qtable) = luma_qtable {
                comp.set_luma_qtable(&qtable);
            }
            if let Some(qtable) = chroma_qtable {
                comp.set_chroma_qtable(&qtable);
            }
        }
    }

    /// 写入 EXIF 元数据（如果启用 metadata 特性）
    #[cfg(feature = "metadata")]
    fn write_exif_metadata(
        image: &Image,
        comp: &mut mozjpeg::compress::Started<WriteAdapter<impl ZByteWriterTrait>>,
    ) {
        use exif::experimental::Writer;

        if let Some(metadata) = &image.metadata().exif() {
            let mut writer = Writer::new();
            // EXIF 数据头：'Exif\0\0'
            let mut buf = std::io::Cursor::new(b"Exif\x00\x00".to_vec());
            buf.set_position(6); // 跳过头部

            for metadatum in *metadata {
                writer.push_field(metadatum);
            }

            if writer.write(&mut buf, false).is_ok() {
                // 将 EXIF 写入 APP1 段
                comp.write_marker(mozjpeg::Marker::APP(1), buf.get_ref());
            } else {
                log::warn!("Writing exif failed");
            }
        }
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
    ) -> std::result::Result<usize, ImageErrors> {
        let (width, height) = image.dimensions();
        let data = &image.flatten_to_u8()[0];

        // 使用 catch_unwind 捕获 MozJpeg 可能的 panic
        std::panic::catch_unwind(AssertUnwindSafe(|| -> std::result::Result<usize, ImageErrors> {
            let input_colorspace = map_colorspace(image.colorspace());

            // 创建并配置压缩器
            let mut comp = mozjpeg::Compress::new(input_colorspace);
            self.configure_compressor(&mut comp, width, height, input_colorspace);
            self.apply_qtable(&mut comp);

            // 创建写入适配器并开始压缩
            let writer = WriteAdapter {
                inner: sink,
                bytes_written: 0,
            };
            let mut comp = comp.start_compress(writer)?;

            // 写入 EXIF 元数据（如果启用）
            #[cfg(feature = "metadata")]
            Self::write_exif_metadata(image, &mut comp);

            // 写入图像扫描线并完成压缩
            comp.write_scanlines(data)?;
            Ok(comp.finish()?.bytes_written)
        }))
        .map_err(|err| {
            // 将 panic 转换为 ImageErrors
            let msg = if let Ok(mut err) = err.downcast::<String>() {
                mem::take(&mut *err)
            } else {
                "Unknown error occurred during encoding".to_string()
            };
            ImageErrors::EncodeErrors(zune_image::errors::ImgEncodeErrors::Generic(msg))
        })?
    }

    /// 返回支持的输入色彩空间列表
    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[
            ColorSpace::Luma,  // 灰度
            ColorSpace::RGBA,  // RGBA
            ColorSpace::RGB,   // RGB
            ColorSpace::YCCK,  // YCCK
            ColorSpace::CMYK,  // CMYK
            ColorSpace::BGR,   // BGR
            ColorSpace::BGRA,  // BGRA
            ColorSpace::ARGB,  // ARGB
            ColorSpace::YCbCr, // YCbCr
        ]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::JPEG
    }

    /// 返回支持的位深度
    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Eight, BitDepth::Sixteen]
    }

    /// 根据输入位深度返回默认输出位深度
    fn default_depth(&self, depth: BitDepth) -> BitDepth {
        match depth {
            BitDepth::Sixteen | BitDepth::Float32 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        }
    }
}
