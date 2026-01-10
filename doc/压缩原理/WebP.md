# WebP 压缩原理

WebP 是 Google 开发的现代图像格式，支持有损和无损压缩。在相同视觉质量下，WebP 通常比 JPEG 小 25-34%，比 PNG 小 26%。

---

## 一、WebP 编码器

### 1.1 核心原理

WebP 基于 VP8 视频编解码器的帧内编码技术，结合了多种先进的压缩算法。

### 1.2 压缩模式

#### 1.2.1 有损压缩 (Lossy)

```rust
WebPOptions {
    lossless: 0,  // 有损模式
    quality: 80.0,
    ...
}
```

有损模式使用类似 JPEG 的技术，但更高效：
- 预测编码
- DCT 变换
- 算术编码（比霍夫曼更高效）

#### 1.2.2 无损压缩 (Lossless)

```rust
WebPOptions {
    lossless: 1,  // 无损模式
    quality: 75.0,  // 控制压缩努力程度
    ...
}
```

无损模式使用完全不同的技术：
- 预测变换
- 颜色变换
- 减色变换
- LZ77 编码

### 1.3 技术手段

#### 1.3.1 压缩方法 (Method)

```rust
method: 6,  // 0-6，6 最慢但压缩最好
```

| 方法 | 速度 | 压缩率 | 适用场景 |
|------|------|--------|----------|
| 0 | 最快 | 最低 | 实时处理 |
| 4 | 平衡 | 中等 | 一般用途 |
| 6 | 最慢 | 最高 | 最终发布 |

#### 1.3.2 图像类型提示 (Image Hint)

```rust
pub enum WebPImageHint {
    Default = 0,   // 自动检测
    Picture = 1,   // 室内场景/数字图片
    Photo = 2,     // 户外自然照片
    Graph = 3,     // 图形/图表/线条艺术
}
```

提示编码器选择最佳压缩策略。

#### 1.3.3 空间噪声整形 (SNS)

```rust
sns_strength: 80,  // 0-100
```

SNS (Spatial Noise Shaping) 在复杂区域分配更多比特，在平滑区域分配更少：
- 较高值在平滑区域保持更好质量
- 减少可见的压缩伪影

#### 1.3.4 滤波器配置

```rust
filter_strength: 50,    // 去块滤波器强度 0-100
filter_sharpness: 0,    // 锐度 0-7，0 最锐利
filter_type: 1,         // 0=简单, 1=强
autofilter: 1,          // 自动调整滤波器
```

去块滤波器减少块效应，但过强会模糊细节。

#### 1.3.5 Alpha 通道处理

```rust
alpha_compression: 1,   // 压缩 Alpha 通道
alpha_filtering: 2,     // 0=无, 1=快速, 2=最佳
alpha_quality: 80,      // Alpha 通道质量
```

WebP 原生支持透明度，Alpha 通道可以独立压缩。

#### 1.3.6 多遍分析

```rust
pass: 10,  // 1-10，更多遍数找到更好压缩
```

多遍分析可以找到更优的压缩参数，但会增加编码时间。

#### 1.3.7 锐利 YUV 转换

```rust
use_sharp_yuv: 1,  // 开启锐利 YUV 转换
```

更锐利但更慢的 RGB→YUV 转换，保持边缘清晰。

#### 1.3.8 近无损模式

```rust
near_lossless: 100,  // 0-100，100=完全无损
```

仅在无损模式下有效，允许轻微有损以获得更好压缩。

### 1.4 动画支持

WebP 原生支持动画，类似 GIF 但更高效：

```rust
if image.is_animated() {
    let mut encoder = webp::AnimEncoder::new(width, height, &options);
    encoder.set_bgcolor([0, 0, 0, 0]);
    encoder.set_loop_count(frames.len() as i32);
    
    for frame in frames {
        encoder.add_frame(frame);
    }
    
    let result = encoder.encode();
}
```

### 1.5 默认配置

```rust
WebPOptions {
    lossless: 0,            // 有损压缩
    quality: 80.0,          // 视觉无损阈值
    method: 6,              // 最大压缩
    image_hint: Default,    // 自动选择
    segments: 4,            // 分段数量
    sns_strength: 80,       // SNS 强度
    filter_strength: 50,    // 滤波器强度
    filter_sharpness: 0,    // 保持锐利
    filter_type: 1,         // 强滤波器
    autofilter: 1,          // 自动调整
    alpha_compression: 1,   // 压缩 Alpha
    alpha_filtering: 2,     // 最佳 Alpha 滤波
    alpha_quality: 80,      // Alpha 质量
    pass: 10,               // 10 遍分析
    thread_level: 1,        // 多线程
    use_sharp_yuv: 1,       // 锐利 YUV
}
```

### 1.6 代码位置

- 编码器: `crates/core/src/codecs/webp/encoder/webp.rs`
- 解码器: `crates/core/src/codecs/webp/decoder.rs`
- 配置选项: `crates/core/src/codecs/webp/encoder/options.rs`

---

## 二、WebP vs JPEG vs PNG

| 特性 | WebP 有损 | WebP 无损 | JPEG | PNG |
|------|-----------|-----------|------|-----|
| 透明度 | ✅ | ✅ | ❌ | ✅ |
| 动画 | ✅ | ✅ | ❌ | ❌ |
| 照片压缩 | 优秀 | 良好 | 良好 | 差 |
| 图形压缩 | 良好 | 优秀 | 差 | 优秀 |
| 浏览器支持 | 广泛 | 广泛 | 全部 | 全部 |

---

## 三、质量与压缩率关系

| 质量 | 视觉效果 | 相比 JPEG | 适用场景 |
|------|----------|-----------|----------|
| 90-100 | 几乎无损 | 小 20-25% | 高清图片 |
| 75-90 | 高质量 | 小 25-30% | 网页图片（推荐） |
| 50-75 | 良好 | 小 30-35% | 一般用途 |
| < 50 | 可接受 | 小 35-40% | 缩略图 |

---

## 四、依赖库

| 库 | 用途 | 特点 |
|----|------|------|
| `webp` | WebP 编解码 | libwebp 的 Rust 绑定 |
| `libwebp-sys` | 底层 FFI | Google 官方 libwebp |
