# AVIF 压缩原理

AVIF (AV1 Image File Format) 是基于 AV1 视频编解码器的现代图像格式，提供卓越的压缩效率。在相同视觉质量下，AVIF 通常比 JPEG 小 50%，比 WebP 小 20%。

---

## 一、AVIF 编码器 (ravif)

### 1.1 核心原理

AVIF 使用 AV1 视频编解码器的帧内编码技术。AV1 是由开放媒体联盟 (AOMedia) 开发的开源、免版税编解码器，代表了当前最先进的压缩技术。

### 1.2 技术手段

#### 1.2.1 质量控制

```rust
pub quality: f32,  // 1-100
```

| 质量范围 | 视觉效果 | 适用场景 |
|----------|----------|----------|
| 80-100 | 几乎无损 | 专业摄影、存档 |
| 60-80 | 高质量 | 网页高清图（推荐） |
| 40-60 | 良好 | 一般网页图 |
| < 40 | 可接受 | 缩略图、预览 |

#### 1.2.2 Alpha 通道质量

```rust
pub alpha_quality: Option<f32>,  // 1-100
```

Alpha 通道可以使用独立的质量设置：
- 透明通道对视觉影响较小
- 可以使用较低质量以获得更好压缩
- 设置为 `None` 时跟随主质量

```rust
// 默认配置
alpha_quality: Some(75.0),  // 比主质量略低
```

#### 1.2.3 压缩速度 (Effort)

```rust
pub speed: u8,  // 1-10
```

| 速度 | 编码时间 | 压缩率 | 适用场景 |
|------|----------|--------|----------|
| 1 | 最慢 | 最高 | 最终发布 |
| 2 | 较慢 | 很高 | 推荐默认 |
| 4 | 平衡 | 良好 | 批量处理 |
| 6 | 较快 | 中等 | 快速预览 |
| 10 | 最快 | 最低 | 实时处理 |

```rust
// 默认使用速度 2，在压缩率和时间之间取得较好平衡
speed: 2,
```

#### 1.2.4 内部颜色模型

```rust
pub color_space: AvifColorSpace,  // YCbCr 或 RGB
```

| 颜色模型 | 说明 | 适用场景 |
|----------|------|----------|
| `YCbCr` | 标准模式 | 大多数图像（推荐） |
| `RGB` | 保留原始 RGB | 需要精确颜色的场景 |

YCbCr 是 AVIF 的标准内部格式，提供最佳压缩效率。

#### 1.2.5 Alpha 颜色模式

```rust
pub alpha_color_mode: AlphaColorMode,
```

| 模式 | 说明 | 效果 |
|------|------|------|
| `UnassociatedClean` | 清理透明区域颜色 | 提高压缩率（推荐） |
| `UnassociatedDirty` | 保留透明区域原始颜色 | 保持原始数据 |
| `Premultiplied` | 预乘 Alpha | 特定渲染需求 |

```rust
// 默认清理透明区域，提高压缩率
alpha_color_mode: AlphaColorMode::UnassociatedClean,
```

### 1.3 编码流程

```rust
let encoder = ravif::Encoder::new()
    .with_quality(self.options.quality)
    .with_alpha_quality(self.options.alpha_quality.unwrap_or(self.options.quality))
    .with_speed(self.options.speed)
    .with_internal_color_model(self.options.color_space)
    .with_alpha_color_mode(self.options.alpha_color_mode);

match image.colorspace() {
    ColorSpace::RGB => {
        let img = Img::new(data.as_rgb(), width, height);
        encoder.encode_rgb(img)
    }
    ColorSpace::RGBA => {
        let img = Img::new(data.as_rgba(), width, height);
        encoder.encode_rgba(img)
    }
}
```

### 1.4 默认配置

```rust
AvifOptions {
    quality: 78.0,                              // 视觉无损阈值
    alpha_quality: Some(75.0),                  // Alpha 略低
    speed: 2,                                   // 较好的压缩率
    color_space: ravif::ColorModel::YCbCr,      // 标准颜色模型
    alpha_color_mode: AlphaColorMode::UnassociatedClean,  // 清理透明区域
}
```

### 1.5 代码位置

- 编码器: `crates/core/src/codecs/avif/encoder/ravif.rs`
- 解码器: `crates/core/src/codecs/avif/decoder.rs`
- 配置选项: `crates/core/src/codecs/avif/encoder/options.rs`

---

## 二、AV1 编码技术

AVIF 继承了 AV1 视频编解码器的先进技术：

### 2.1 预测编码

- **帧内预测**: 使用相邻像素预测当前块
- **多种预测模式**: 支持 50+ 种方向预测模式
- **递归分区**: 支持从 128×128 到 4×4 的灵活块大小

### 2.2 变换编码

- **多种变换类型**: DCT、ADST、Identity 等
- **变换大小**: 从 4×4 到 64×64
- **变换类型选择**: 根据内容自动选择最佳变换

### 2.3 熵编码

- **符号编码**: 使用高效的算术编码
- **上下文建模**: 根据周围像素调整概率模型
- **自适应概率**: 编码过程中动态更新概率

### 2.4 环路滤波

- **去块滤波**: 减少块边界伪影
- **CDEF**: 约束方向增强滤波器
- **环路恢复**: 自适应 Wiener 滤波和自引导滤波

---

## 三、AVIF vs 其他格式

| 特性 | AVIF | WebP | JPEG | PNG |
|------|------|------|------|-----|
| 压缩效率 | 最高 | 高 | 中 | 低 |
| 透明度 | ✅ | ✅ | ❌ | ✅ |
| 动画 | ✅ | ✅ | ❌ | ❌ |
| HDR | ✅ | ❌ | ❌ | ❌ |
| 10/12位色深 | ✅ | ❌ | ❌ | ✅ |
| 编码速度 | 慢 | 中 | 快 | 快 |
| 浏览器支持 | 广泛 | 广泛 | 全部 | 全部 |

---

## 四、质量与压缩率关系

| 质量 | 视觉效果 | 相比 JPEG | 相比 WebP |
|------|----------|-----------|-----------|
| 85-100 | 几乎无损 | 小 40-50% | 小 15-20% |
| 70-85 | 高质量 | 小 50-60% | 小 20-25% |
| 50-70 | 良好 | 小 55-65% | 小 25-30% |
| < 50 | 可接受 | 小 60-70% | 小 30-35% |

---

## 五、依赖库

| 库 | 用途 | 特点 |
|----|------|------|
| `ravif` | AVIF 编码 | 基于 rav1e 的 Rust 原生编码器 |
| `libavif` | AVIF 解码 | 支持多种 AV1 解码器后端 |
| `rav1e` | AV1 编码 | Rust 原生 AV1 编码器 |
| `rgb` | 像素处理 | RGB/RGBA 像素类型 |

---

## 六、注意事项

1. **编码速度**: AVIF 编码比 JPEG/WebP 慢很多，建议用于最终发布而非实时处理
2. **内存使用**: 高质量编码可能需要较多内存
3. **兼容性**: 虽然浏览器支持广泛，但某些旧版本可能不支持
4. **HDR 支持**: AVIF 支持 HDR 和宽色域，但需要相应的显示设备
