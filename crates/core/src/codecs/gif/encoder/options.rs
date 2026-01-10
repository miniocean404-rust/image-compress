use serde::{Deserialize, Serialize};

/// GIF 编码选项
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GifOptions {
    /// 有损压缩级别 (0-200)
    /// 0 表示无损压缩
    /// 值越高，压缩率越高，但质量越低
    pub lossy: u8,

    /// 优化级别 (1-3)
    /// 1: 最快，最小优化
    /// 2: 中等优化
    /// 3: 最慢，最大优化
    pub optimize_level: u8,

    /// 是否减少颜色数量
    pub reduce_colors: bool,

    /// 最大颜色数量 (2-256)
    pub max_colors: u16,
}

impl Default for GifOptions {
    fn default() -> Self {
        Self {
            // 默认使用有损压缩 (lossy=40)，人眼几乎无法察觉
            // 官方默认 20，40 在人眼无感知范围内且压缩效果更好
            lossy: 40,
            // 默认使用最高优化级别
            optimize_level: 3,
            reduce_colors: false,
            max_colors: 256,
        }
    }
}

impl GifOptions {
    /// 创建无损压缩选项
    pub fn lossless() -> Self {
        Self {
            lossy: 0,
            optimize_level: 3,
            reduce_colors: false,
            max_colors: 256,
        }
    }

    /// 创建有损压缩选项
    ///
    /// # 参数
    /// - quality: 质量 (0-100)，100 表示最高质量
    pub fn lossy(quality: u8) -> Self {
        // 将 quality (0-100) 转换为 lossy (0-200)
        // quality 100 -> lossy 0 (无损)
        // quality 0 -> lossy 200 (最大有损)
        let lossy = ((100 - quality.min(100)) as u16 * 2) as u8;

        Self {
            lossy,
            optimize_level: 3,
            reduce_colors: false,
            max_colors: 256,
        }
    }

    /// 创建最大压缩选项（人眼无感知）
    ///
    /// 使用 lossy=50 的有损压缩，这是人眼几乎无法察觉的临界值，
    /// 同时启用最高优化级别以获得最佳压缩效果。
    /// 根据社区经验，lossy 在 20-65 范围内通常人眼难以察觉，
    /// 50 是在压缩率和质量之间的最佳平衡点。
    pub fn max_compression() -> Self {
        Self {
            // lossy 50 是人眼难以察觉的临界值
            // 超过此值可能会出现可见的色带和伪影
            lossy: 50,
            optimize_level: 3,
            reduce_colors: false,
            max_colors: 256,
        }
    }

    /// 设置有损压缩级别
    pub fn with_lossy(mut self, lossy: u8) -> Self {
        self.lossy = lossy.min(200);
        self
    }

    /// 设置优化级别
    pub fn with_optimize_level(mut self, level: u8) -> Self {
        self.optimize_level = level.clamp(1, 3);
        self
    }

    /// 设置是否减少颜色
    pub fn with_reduce_colors(mut self, reduce: bool) -> Self {
        self.reduce_colors = reduce;
        self
    }

    /// 设置最大颜色数量
    pub fn with_max_colors(mut self, colors: u16) -> Self {
        self.max_colors = colors.clamp(2, 256);
        self
    }
}
