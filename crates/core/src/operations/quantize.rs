use imagequant::Histogram;
use rgb::FromSlice;
use zune_core::{bit_depth::BitType, colorspace::ColorSpace};
use zune_image::{
    channel::Channel,
    errors::{ImageErrors, ImageOperationsErrors},
    traits::OperationsTrait,
};

/// 减少图像调色板, 可对任意图片进行操作，但必须是 RGBA 格式
pub struct Quantize {
    quality: u8,
    dithering: Option<f32>,
}

impl Quantize {
    /// 创建一个新的量化操作
    ///
    /// # 参数
    /// - quality: 调色板的结果质量
    /// - dithering: 最终图像的整体“平滑度”
    #[must_use]
    pub fn new(quality: u8, dithering: Option<f32>) -> Self {
        Self { quality, dithering }
    }
}

impl OperationsTrait for Quantize {
    fn name(&self) -> &'static str {
        "quantize"
    }

    fn execute_impl(&self, image: &mut zune_image::image::Image) -> Result<(), ImageErrors> {
        let (src_width, src_height) = image.dimensions();
        let channel_len = src_width * src_height * image.depth().size_of();

        let mut liq = imagequant::new();

        liq.set_quality(0, self.quality)
            .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

        let mut histogram = Histogram::new(&liq);

        let mut frames = image
            .frames_ref()
            .iter()
            .map(|frame| {
                let mut img = liq
                    .new_image(
                        frame.flatten(image.colorspace()).as_rgba(),
                        src_width,
                        src_height,
                        0.0,
                    )
                    .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

                histogram
                    .add_image(&liq, &mut img)
                    .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

                Ok::<imagequant::Image, ImageErrors>(img)
            })
            .collect::<Result<Vec<imagequant::Image>, ImageErrors>>()?;

        let mut res = histogram
            .quantize(&liq)
            .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

        if let Some(dithering) = self.dithering {
            res.set_dithering_level(dithering)
                .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;
        }

        frames
            .iter_mut()
            .zip(image.frames_mut())
            .try_for_each(|(img, frame)| {
                let (palette, pixels) = res
                    .remapped(img)
                    .map_err(|e| ImageOperationsErrors::GenericString(e.to_string()))?;

                let channels = pixels
                    .iter()
                    .map(|px| {
                        let px = palette[*px as usize];
                        (px.r, px.g, px.b, px.a)
                    })
                    .enumerate()
                    .fold(
                        vec![Channel::new_with_bit_type(channel_len, BitType::U8); 4],
                        |mut acc, (idx, px)| {
                            unsafe {
                                acc[0].alias_mut()[idx] = px.0;
                                acc[1].alias_mut()[idx] = px.1;
                                acc[2].alias_mut()[idx] = px.2;
                                acc[3].alias_mut()[idx] = px.3;
                            }

                            acc
                        },
                    );

                frame.set_channels(channels);

                Ok::<(), ImageErrors>(())
            })?;

        Ok(())
    }

    fn supported_types(&self) -> &'static [BitType] {
        &[BitType::U8]
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGBA]
    }
}
