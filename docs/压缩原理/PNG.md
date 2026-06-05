# PNG 压缩原理

PNG (Portable Network Graphics) 是一种无损压缩的位图图像格式。本项目使用两种互补的压缩策略：**OxiPNG 无损压缩** 和 **ImageQuant 有损压缩**。

---

## 一、OxiPNG 无损压缩

### 1.1 核心原理

OxiPNG 是一个多线程无损 PNG 压缩优化器，通过尝试不同的过滤器和压缩参数来找到最小的文件大小，同时保持图像**完全无损**。

### 1.2 技术手段

#### 1.2.1 过滤器优化 (Filter Strategy)

PNG 在压缩前会对每行像素应用过滤器，以提高后续 DEFLATE 压缩的效率。OxiPNG 会尝试多种过滤策略：

| 过滤器 | 原理 | 适用场景 |
|--------|------|----------|
| `None` | 不做任何处理 | 随机噪声图像 |
| `Sub` | 当前像素减去左侧像素 | 水平渐变 |
| `Up` | 当前像素减去上方像素 | 垂直渐变 |
| `Average` | 减去左侧和上方像素的平均值 | 平滑渐变 |
| `Paeth` | 使用 Paeth 预测算法 | 复杂图像 |
| `Entropy` | 基于熵选择最佳过滤器 | 通用 |
| `Bigrams` | 基于双字节模式选择 | 通用 |

```rust
// 默认配置尝试多种过滤策略
opts.filters = indexset![None, Sub, Entropy, Bigrams];
```

#### 1.2.2 DEFLATE 压缩优化

PNG 使用 DEFLATE 算法进行数据压缩。OxiPNG 使用 **libdeflater** 库，支持最高压缩级别 12：

```rust
opts.deflater = Deflater::Libdeflater { compression: 12 };
```

#### 1.2.3 位深度和颜色类型减少

自动检测并减少不必要的位深度和颜色通道：

| 优化类型 | 说明 | 效果 |
|----------|------|------|
| `bit_depth_reduction` | 16位 → 8位（如果无损） | 减少 50% 数据量 |
| `color_type_reduction` | RGBA → RGB（如果无透明） | 减少 25% 数据量 |
| `palette_reduction` | 减少调色板颜色数 | 减少调色板大小 |
| `grayscale_reduction` | RGB → 灰度（如果是灰度图） | 减少 66% 数据量 |

#### 1.2.4 透明像素优化

```rust
opts.optimize_alpha = true;
```

允许修改完全透明像素的 RGB 值，因为这些像素不可见，但可以提高压缩率。

#### 1.2.5 交错模式

```rust
opts.interlace = Some(false);
```

关闭交错（Adam7）模式，非交错图像通常压缩更好。

#### 1.2.6 元数据剥离

```rust
opts.strip = StripChunks::Safe;
```

移除非显示相关的元数据块（如注释、时间戳），但保留颜色配置文件等重要信息。

### 1.3 代码位置

- 编码器: `crates/core/src/codecs/png/encoder/oxipng.rs`
- 配置选项: `crates/core/src/codecs/png/encoder/oxipng_options.rs`

---

## 二、ImageQuant 有损压缩

### 2.1 核心原理

ImageQuant 通过**颜色量化**将 PNG 转换为 8 位调色板图像（最多 256 色），可以大幅减小文件大小（通常 50-80%），同时保持良好的视觉质量。

### 2.2 技术手段

#### 2.2.1 颜色量化算法

ImageQuant 使用改进的中位切分算法和感知颜色空间，将真彩色图像（1600万色）减少到 256 色：

```rust
let mut attr = imagequant::new();
attr.set_quality(min_quality, max_quality)?;
let mut quantize_res = attr.quantize(&mut img)?;
```

#### 2.2.2 质量控制

| 参数 | 范围 | 说明 |
|------|------|------|
| `min_quality` | 0-100 | 最低可接受质量，低于此值返回错误 |
| `max_quality` | 0-100 | 目标质量上限，推荐 80-90 |
| `speed` | 1-10 | 1=最慢最高质量，10=最快 |

```rust
// 默认配置
ImageQuantOptions {
    min_quality: 0,      // 允许任何质量
    max_quality: 85,     // 视觉无损阈值
    speed: 1,            // 最高质量
    ...
}
```

#### 2.2.3 抖动 (Dithering)

抖动通过在相邻像素间交替使用不同颜色来模拟中间色调，减少色带效应：

```rust
quantize_res.set_dithering_level(0.75)?;  // 0.0-1.0
```

| 抖动级别 | 效果 |
|----------|------|
| 0.0 | 无抖动，最小文件大小，可能有色带 |
| 0.5 | 中等抖动 |
| 1.0 | 完全抖动，最平滑渐变，文件较大 |

#### 2.2.4 色阶化 (Posterization)

```rust
attr.set_min_posterization(0)?;  // 0-4
```

忽略最低有效位，用于生成复古风格或进一步减少颜色。

#### 2.2.5 Gamma 校正

```rust
quantize_res.set_output_gamma(0.45455)?;  // sRGB gamma ≈ 1/2.2
```

确保颜色在不同显示设备上的一致性。

### 2.3 输出编码

量化后使用 **lodepng** 库编码为 PNG 格式：

```rust
let mut enc = lodepng::Encoder::new();
enc.info_raw_mut().set_bitdepth(8);
enc.set_palette(palette)?;
enc.encode(pixels.as_slice(), width, height)
```

### 2.4 代码位置

- 编码器: `crates/core/src/codecs/png/encoder/imagequant.rs`
- 配置选项: `crates/core/src/codecs/png/encoder/imagequant_options.rs`

---

## 三、压缩策略选择

| 场景 | 推荐方案 | 原因 |
|------|----------|------|
| 需要完全无损 | OxiPNG | 像素级精确 |
| 照片/复杂图像 | ImageQuant | 压缩率高，视觉差异小 |
| 图标/简单图形 | OxiPNG | 颜色少，无损压缩效果好 |
| 截图/UI | ImageQuant | 大幅减小文件大小 |

---

## 四、依赖库

| 库 | 用途 | 特点 |
|----|------|------|
| `oxipng` | 无损 PNG 优化 | 多线程、多策略尝试 |
| `imagequant` | 颜色量化 | 感知颜色空间、高质量抖动 |
| `lodepng` | PNG 编码 | 轻量、支持调色板 |
| `libdeflater` | DEFLATE 压缩 | 高压缩率、快速 |
