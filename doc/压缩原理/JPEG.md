# JPEG 压缩原理

JPEG (Joint Photographic Experts Group) 是一种有损压缩的图像格式，特别适合照片和复杂图像。本项目使用 **MozJpeg** 编码器，这是 Mozilla 开发的高效 JPEG 编码器。

---

## 一、MozJpeg 编码器

### 1.1 核心原理

MozJpeg 是基于 libjpeg-turbo 的改进版本，通过更智能的量化和编码策略，在相同视觉质量下生成更小的文件。

### 1.2 技术手段

#### 1.2.1 DCT 变换 (离散余弦变换)

JPEG 压缩的核心是将图像从空间域转换到频率域：

1. 将图像分割为 8×8 像素块
2. 对每个块应用 DCT 变换
3. 高频分量（细节）可以更激进地量化

#### 1.2.2 量化表优化

量化是 JPEG 有损压缩的关键步骤。MozJpeg 提供多种优化的量化表：

| 量化表类型     | 优化目标          | 适用场景           |
| -------------- | ----------------- | ------------------ |
| `AnnexK_Luma`  | JPEG 标准亮度表   | 通用               |
| `MSSSIM_Luma`  | MS-SSIM 感知质量  | 高质量照片（推荐） |
| `PSNRHVS_Luma` | PSNR-HVS 人眼感知 | 平衡质量和大小     |
| `Flat`         | 平坦量化          | 特殊需求           |
| `NRobidoux`    | Robidoux 优化     | 特定场景           |

```rust
// 高质量模式自动使用 MSSSIM 量化表
if self.options.auto_qtable && self.options.quality >= 80.0 {
    let luma_qtable = build_qtable(Some(&QtableOptimize::MSSSIM_Luma), quality);
    let chroma_qtable = build_chroma_qtable(Some(&QtableOptimizeChroma::MSSSIM_Chroma), quality);
    comp.set_luma_qtable(&luma_qtable);
    comp.set_chroma_qtable(&chroma_qtable);
}
```

#### 1.2.3 色度子采样 (Chroma Subsampling)

## 基本原理

人眼对**亮度（Y）的敏感度远高于对色度（Cb/Cr，即颜色信息）**的敏感度。色度子采样利用这一特性，保留完整的亮度信息，但降低色度信息的分辨率。

### 4:2:0 的含义

采样比例用 J:a:b 三个数字表示（基于 4×2 像素块）：

| 格式  | 含义                                                  | 色度数据量      |
| ----- | ----------------------------------------------------- | --------------- |
| 4:4:4 | 每个像素都有完整的 Y、Cb、Cr, 每像素独立色度          | 100% (无子采样) |
| 4:2:2 | 水平方向每 2 个像素共享色度                           | 66%             |
| 4:2:0 | 水平+垂直方向每 4 个像素共享色度, 4 像素共享 1 组色度 | 50%             |

### 为什么 4:2:0 影响大

以 1920×1080 图像为例：

- 4:4:4: Y(2M) + Cb(2M) + Cr(2M) = 6M 采样点
- 4:2:0: Y(2M) + Cb(0.5M) + Cr(0.5M) = 3M 采样点

数据量直接减半，这是在 DCT 压缩之前就完成的，所以对最终文件大小影响巨大。

## 适用场景

| 格式  | 适用场景                                   |
| ----- | ------------------------------------------ |
| 4:4:4 | 文字截图、图形设计、需要精确颜色边缘       |
| 4:2:2 | 视频制作中间格式                           |
| 4:2:0 | 照片、视频最终输出（人眼几乎察觉不到差异） |

对于照片类图像，4:2:0 通常是最佳选择——文件小且视觉质量损失极小。但对于有锐利颜色边缘的图像（如文字、图标），4:4:4 能避免色彩边缘模糊。

```rust
// 根据质量自动选择子采样
fn auto_chroma_subsample(quality: f32) -> (u8, u8) {
    match quality {
        q if q > 90.0 => (1, 1), // 4:4:4 无子采样
        q if q > 70.0 => (2, 1), // 4:2:2
        _ => (2, 2),             // 4:2:0 最大压缩
    }
}
```

#### 1.2.4 渐进式编码 (Progressive Mode)

渐进式 JPEG 允许图像逐步显示，从模糊到清晰：

```rust
if self.options.progressive {
    comp.set_progressive_mode();
}
```

**优势**：

- 用户体验更好（快速预览）
- 通常文件更小（5-10%）
- 支持扫描顺序优化

#### 1.2.5 霍夫曼编码优化

```rust
comp.set_optimize_coding(true);
```

优化霍夫曼编码表，根据实际数据分布生成最优编码，而非使用标准表。

#### 1.2.6 Trellis 量化

Trellis 量化是 MozJpeg 的核心优势之一，通过多遍优化找到最佳量化系数：

```rust
// Trellis 多遍优化
comp.set_use_scans_in_trellis(true);

// 渐进式扫描优化
comp.set_optimize_scans(true);
```

**原理**：

- 将量化视为路径搜索问题
- 使用动态规划找到最优量化路径
- 可减少 3-5% 文件大小

#### 1.2.7 平滑因子

```rust
comp.set_smoothing_factor(0);  // 0-100
```

减少块效应，但会轻微模糊图像。默认为 0（不平滑）。

### 1.3 色彩空间转换

MozJpeg 支持多种输入色彩空间：

```rust
fn map_colorspace(cs: ColorSpace) -> mozjpeg::ColorSpace {
    match cs {
        ColorSpace::RGB => mozjpeg::ColorSpace::JCS_RGB,
        ColorSpace::RGBA => mozjpeg::ColorSpace::JCS_EXT_RGBA,
        ColorSpace::YCbCr => mozjpeg::ColorSpace::JCS_YCbCr,
        ColorSpace::Luma => mozjpeg::ColorSpace::JCS_GRAYSCALE,
        ColorSpace::CMYK => mozjpeg::ColorSpace::JCS_CMYK,
        // ...
    }
}
```

输出通常转换为 YCbCr 色彩空间，这是 JPEG 的标准内部格式。

### 1.4 默认配置

```rust
MozJpegOptions {
    quality: 70.0,              // 质量因子
    progressive: true,          // 渐进式编码
    optimize_coding: true,      // 优化霍夫曼表
    smoothing: 0,               // 不平滑
    color_space: JCS_YCbCr,     // YCbCr 输出
    trellis_multipass: true,    // Trellis 多遍优化
    optimize_scans: true,       // 扫描顺序优化
    auto_chroma_subsample: true,// 自动色度子采样
    auto_qtable: true,          // 自动量化表选择
}
```

### 1.5 代码位置

- 编码器: `crates/core/src/codecs/jpeg/encoder/mozjpeg.rs`
- 配置选项: `crates/core/src/codecs/jpeg/encoder/options.rs`

---

## 二、JPEG XL 支持

项目还支持 JPEG XL 格式（通过 zune_image）：

```rust
// crates/core/src/codecs/jpeg/encoder/jpeg_xl.rs
pub fn encode_mem_jxl(buf: &Vec<u8>, options: EncoderOptions) -> Result<Vec<u8>, ImageErrors>
```

JPEG XL 是新一代图像格式，支持无损和有损压缩，但文件较大。

---

## 三、质量与压缩率关系

| 质量   | 视觉效果 | 典型压缩率 | 适用场景   |
| ------ | -------- | ---------- | ---------- |
| 90-100 | 几乎无损 | 10:1       | 专业摄影   |
| 80-90  | 高质量   | 15:1       | 网页高清图 |
| 60-80  | 良好     | 20:1       | 一般网页图 |
| 40-60  | 可接受   | 30:1       | 缩略图     |
| < 40   | 明显失真 | 50:1+      | 极限压缩   |

---

## 四、MozJpeg vs 标准 JPEG

| 特性         | 标准 JPEG | MozJpeg      |
| ------------ | --------- | ------------ |
| Trellis 量化 | ❌        | ✅           |
| 优化量化表   | 固定      | 多种可选     |
| 渐进式优化   | 基础      | 高级扫描优化 |
| 文件大小     | 基准      | 小 5-15%     |
| 编码速度     | 快        | 较慢         |

---

## 五、依赖库

| 库                | 用途      | 特点                   |
| ----------------- | --------- | ---------------------- |
| `mozjpeg`         | JPEG 编码 | Trellis 量化、多种优化 |
| `mozjpeg::qtable` | 量化表    | 多种感知优化量化表     |
| `zune_image`      | 图像处理  | 统一的图像接口         |
