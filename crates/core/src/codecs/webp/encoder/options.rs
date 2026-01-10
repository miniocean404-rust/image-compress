/// WebP 图像提示类型
///
/// 提示编码器图像的类型，以便选择最佳的压缩策略。
#[derive(Debug, Clone, Copy, Default)]
#[repr(i32)]
pub enum WebPImageHint {
    /// 默认提示，自动检测
    #[default]
    Default = 0,
    /// 室内场景等数字图片
    Picture = 1,
    /// 户外自然照片
    Photo = 2,
    /// 图形、图表、线条艺术
    Graph = 3,
}

/// WebP 编码选项
///
/// WebP 是 Google 开发的现代图像格式，支持有损和无损压缩。
/// 在相同视觉质量下，WebP 通常比 JPEG 小 25-34%，比 PNG 小 26%。
///
/// ## 压缩模式
///
/// - **有损压缩** (`lossless = 0`): 适合照片，提供最佳压缩率
/// - **无损压缩** (`lossless = 1`): 适合图形、截图，保持像素精确
///
/// ## 推荐配置
///
/// ### 照片（最大压缩，人眼难以察觉）
/// ```rust
/// WebPOptions {
///     lossless: 0,
///     quality: 82.0,
///     method: 6,
///     image_hint: WebPImageHint::Photo,
///     ..Default::default()
/// }
/// ```
///
/// ### 图形/截图（无损）
/// ```rust
/// WebPOptions {
///     lossless: 1,
///     quality: 75.0,  // 无损模式下控制压缩努力程度
///     method: 6,
///     image_hint: WebPImageHint::Graph,
///     ..Default::default()
/// }
/// ```
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct WebPOptions {
    /// 无损模式: 0 = 有损, 1 = 无损
    ///
    /// 默认值: 0（有损压缩，提供更好的压缩率）
    pub lossless: i32,

    /// 质量因子 `0.0..=100.0`
    ///
    /// 有损模式:
    /// - 100: 最高质量
    /// - 80-85: 高质量，人眼几乎无法察觉差异（推荐）
    /// - 60-80: 良好质量，轻微可见差异
    /// - < 60: 明显压缩痕迹
    ///
    /// 无损模式: 控制压缩努力程度（0=快速, 100=最慢但最小）
    ///
    /// 默认值: 82.0（高质量有损压缩）
    pub quality: f32,

    /// 压缩方法 `0..=6`
    ///
    /// - 0: 最快，文件较大
    /// - 4: 平衡速度和大小
    /// - 6: 最慢，文件最小（推荐用于最终输出）
    ///
    /// 默认值: 6（最大压缩）
    pub method: i32,

    /// 图像类型提示
    ///
    /// 帮助编码器选择最佳压缩策略。
    ///
    /// 默认值: Photo（适合大多数照片）
    pub image_hint: WebPImageHint,

    /// 目标文件大小（字节）
    ///
    /// 设置后会自动调整质量以达到目标大小。
    /// 0 = 不限制。
    ///
    /// 默认值: 0
    pub target_size: i32,

    /// 目标 PSNR（dB）
    ///
    /// 设置后会自动调整质量以达到目标 PSNR。
    /// 0 = 不限制。典型值: 42dB。
    ///
    /// 默认值: 0.0
    pub target_PSNR: f32,

    /// 分段数量 `1..=4`
    ///
    /// 更多分段可以提高质量但增加文件大小。
    ///
    /// 默认值: 4
    pub segments: i32,

    /// 空间噪声整形强度 `0..=100`
    ///
    /// 控制在复杂区域分配更多比特的程度。
    /// 较高值在平滑区域保持更好质量。
    ///
    /// 默认值: 80（增强平滑区域质量）
    pub sns_strength: i32,

    /// 滤波器强度 `0..=100`
    ///
    /// 去块滤波器强度。较高值减少块效应但可能模糊细节。
    ///
    /// 默认值: 50（平衡去块和细节保留）
    pub filter_strength: i32,

    /// 滤波器锐度 `0..=7`
    ///
    /// 0 = 最锐利, 7 = 最不锐利。
    ///
    /// 默认值: 0（保持锐利）
    pub filter_sharpness: i32,

    /// 滤波器类型: 0 = 简单, 1 = 强
    ///
    /// 强滤波器在低比特率下效果更好。
    ///
    /// 默认值: 1（强滤波器）
    pub filter_type: i32,

    /// 自动滤波器: 0 = 关闭, 1 = 开启
    ///
    /// 自动调整滤波器强度。
    ///
    /// 默认值: 1（开启自动调整）
    pub autofilter: i32,

    /// Alpha 通道压缩: 0 = 无, 1 = 压缩
    ///
    /// 默认值: 1（压缩 Alpha）
    pub alpha_compression: i32,

    /// Alpha 滤波: 0 = 无, 1 = 快速, 2 = 最佳
    ///
    /// 默认值: 2（最佳质量）
    pub alpha_filtering: i32,

    /// Alpha 通道质量 `0..=100`
    ///
    /// Alpha 通道对视觉影响较小，可以使用较低质量。
    ///
    /// 默认值: 90（高质量 Alpha）
    pub alpha_quality: i32,

    /// 分析遍数 `1..=10`
    ///
    /// 更多遍数可以找到更好的压缩，但更慢。
    ///
    /// 默认值: 6（良好的压缩/速度平衡）
    pub pass: i32,

    /// 显示压缩数据: 0 = 关闭, 1 = 开启
    ///
    /// 默认值: 0
    pub show_compressed: i32,

    /// 预处理: 0 = 无, 1 = 分段平滑, 2 = 伪随机抖动
    ///
    /// 默认值: 0
    pub preprocessing: i32,

    /// 分区数量 `0..=3`
    ///
    /// 0 = 自动选择。
    ///
    /// 默认值: 0
    pub partitions: i32,

    /// 分区大小限制 `0..=100`
    ///
    /// 限制第一个分区的大小以适应 512KB 限制。
    /// 0 = 无限制。
    ///
    /// 默认值: 0
    pub partition_limit: i32,

    /// 模拟 JPEG 大小: 0 = 关闭, 1 = 开启
    ///
    /// 默认值: 0
    pub emulate_jpeg_size: i32,

    /// 多线程: 0 = 关闭, 1 = 开启
    ///
    /// 默认值: 1（开启多线程）
    pub thread_level: i32,

    /// 低内存模式: 0 = 关闭, 1 = 开启
    ///
    /// 默认值: 0
    pub low_memory: i32,

    /// 近无损质量 `0..=100`
    ///
    /// 仅在无损模式下有效。100 = 完全无损。
    /// 较低值允许轻微有损以获得更好压缩。
    ///
    /// 默认值: 100（完全无损）
    pub near_lossless: i32,

    /// 精确模式: 0 = 关闭, 1 = 开启
    ///
    /// 保留透明区域的 RGB 值。
    ///
    /// 默认值: 0（允许优化透明区域）
    pub exact: i32,

    /// 使用 Delta 调色板: 0 = 关闭, 1 = 开启
    ///
    /// 默认值: 0
    pub use_delta_palette: i32,

    /// 使用锐利 YUV 转换: 0 = 关闭, 1 = 开启
    ///
    /// 更锐利但更慢的 RGB->YUV 转换。
    ///
    /// 默认值: 1（开启，保持锐利）
    pub use_sharp_yuv: i32,

    /// 最小量化值 `0..=100`
    ///
    /// 默认值: 0
    pub qmin: i32,

    /// 最大量化值 `0..=100`
    ///
    /// 默认值: 100
    pub qmax: i32,
}

impl Default for WebPOptions {
    fn default() -> Self {
        Self {
            // 有损压缩，提供更好的压缩率
            lossless: 0,
            // 质量 80: 视觉无损阈值，Google 推荐的默认值
            // 研究表明 WebP 在 quality 75-85 范围内人眼几乎无法察觉差异
            quality: 80.0,
            // 方法 6: 最慢但压缩最好
            method: 6,
            // 自动选择
            image_hint: WebPImageHint::Default,
            target_size: 0,
            target_PSNR: 0.0,
            segments: 4,
            // SNS 强度 80: 增强平滑区域质量
            sns_strength: 80,
            // 滤波器强度 50: 平衡去块和细节
            filter_strength: 50,
            // 保持锐利
            filter_sharpness: 0,
            // 强滤波器
            filter_type: 1,
            // 自动调整滤波器
            autofilter: 1,
            // 压缩 Alpha 通道
            alpha_compression: 1,
            // 最佳 Alpha 滤波
            alpha_filtering: 2,
            // Alpha 通道质量 80: 透明通道对视觉影响较小，可以更激进压缩
            alpha_quality: 80,
            // 10 遍分析: 最大化压缩率
            pass: 10,
            show_compressed: 0,
            preprocessing: 0,
            partitions: 0,
            partition_limit: 0,
            emulate_jpeg_size: 0,
            // 开启多线程
            thread_level: 1,
            low_memory: 0,
            // 完全无损（仅无损模式）
            near_lossless: 100,
            // 允许优化透明区域
            exact: 0,
            use_delta_palette: 0,
            // 开启锐利 YUV 转换
            use_sharp_yuv: 1,
            qmin: 0,
            qmax: 100,
        }
    }
}

impl From<WebPOptions> for webp::WebPConfig {
    fn from(value: WebPOptions) -> Self {
        let mut config = webp::WebPConfig::new().unwrap();

        config.lossless = value.lossless;
        config.quality = value.quality;
        config.method = value.method;
        // 使用 transmute 转换自定义枚举到 libwebp_sys 的枚举
        // 两者的内存布局相同 (都是 i32)
        config.image_hint = unsafe { std::mem::transmute(value.image_hint as i32) };
        config.target_size = value.target_size;
        config.target_PSNR = value.target_PSNR;
        config.segments = value.segments;
        config.sns_strength = value.sns_strength;
        config.filter_strength = value.filter_strength;
        config.filter_sharpness = value.filter_sharpness;
        config.filter_type = value.filter_type;
        config.autofilter = value.autofilter;
        config.alpha_compression = value.alpha_compression;
        config.alpha_filtering = value.alpha_filtering;
        config.alpha_quality = value.alpha_quality;
        config.pass = value.pass;
        config.show_compressed = value.show_compressed;
        config.preprocessing = value.preprocessing;
        config.partitions = value.partitions;
        config.partition_limit = value.partition_limit;
        config.emulate_jpeg_size = value.emulate_jpeg_size;
        config.thread_level = value.thread_level;
        config.low_memory = value.low_memory;
        config.near_lossless = value.near_lossless;
        config.exact = value.exact;
        config.use_delta_palette = value.use_delta_palette;
        config.use_sharp_yuv = value.use_sharp_yuv;
        config.qmin = value.qmin;
        config.qmax = value.qmax;

        config
    }
}
