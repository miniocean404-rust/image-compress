# TIFF 压缩原理

TIFF (Tagged Image File Format) 是一种灵活的位图图像格式，广泛用于专业摄影、出版和存档。本项目**仅支持 TIFF 解码**，不提供压缩编码功能。

---

## 一、TIFF 解码器

### 1.1 核心原理

TIFF 是一种容器格式，支持多种压缩算法和颜色空间。解码器需要处理各种不同的 TIFF 变体。

### 1.2 支持的颜色空间

```rust
let colorspace = match colortype {
    tiff::ColorType::RGB(_) => ColorSpace::RGB,
    tiff::ColorType::RGBA(_) => ColorSpace::RGBA,
    tiff::ColorType::CMYK(_) => ColorSpace::CMYK,
    tiff::ColorType::Gray(_) => ColorSpace::Luma,
    tiff::ColorType::GrayA(_) => ColorSpace::LumaA,
    tiff::ColorType::YCbCr(_) => ColorSpace::YCbCr,
    _ => ColorSpace::Unknown,
};
```

| 颜色类型 | 说明 | 用途 |
|----------|------|------|
| RGB | 红绿蓝三通道 | 一般图像 |
| RGBA | RGB + Alpha | 带透明度图像 |
| CMYK | 青品黄黑四通道 | 印刷 |
| Gray | 灰度 | 黑白图像 |
| GrayA | 灰度 + Alpha | 带透明度灰度图 |
| YCbCr | 亮度色度 | 视频/JPEG |

### 1.3 支持的位深度

```rust
match result {
    tiff::decoder::DecodingResult::U8(data) => Image::from_u8(&data, ...),
    tiff::decoder::DecodingResult::U16(data) => Image::from_u16(&data, ...),
    tiff::decoder::DecodingResult::F32(data) => Image::from_f32(&data, ...),
    _ => Err("Tiff Data format not supported"),
}
```

| 位深度 | 说明 | 用途 |
|--------|------|------|
| 8位 | 标准位深度 | 一般图像 |
| 16位 | 高动态范围 | 专业摄影、医学影像 |
| 32位浮点 | 最高精度 | 科学计算、HDR |

### 1.4 解码流程

```rust
impl<R: BufRead + Seek> TiffDecoder<R> {
    pub fn try_new(source: R) -> Result<Self, ImageErrors> {
        let inner = tiff::decoder::Decoder::new(source)?;
        Ok(Self { inner, dimensions: None, colorspace: ColorSpace::Unknown })
    }
}

impl<R> DecoderTrait for TiffDecoder<R> {
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        // 1. 获取尺寸
        let (width, height) = self.inner.dimensions()?;
        
        // 2. 获取颜色类型
        let colorspace = self.inner.colortype()?;
        
        // 3. 读取图像数据
        let result = self.inner.read_image()?;
        
        // 4. 转换为统一格式
        Ok(Image::from_xxx(&data, width, height, colorspace))
    }
}
```

### 1.5 代码位置

- 解码器: `crates/core/src/codecs/tiff/decoder.rs`
- 模块: `crates/core/src/codecs/tiff/mod.rs`

---

## 二、TIFF 格式特点

### 2.1 支持的压缩算法

TIFF 格式支持多种压缩算法（由 `tiff` 库处理）：

| 压缩算法 | 类型 | 说明 |
|----------|------|------|
| None | 无压缩 | 最大文件，最快读取 |
| LZW | 无损 | 通用无损压缩 |
| PackBits | 无损 | 简单 RLE 压缩 |
| Deflate/ZIP | 无损 | 高效无损压缩 |
| JPEG | 有损 | 照片压缩 |
| CCITT | 无损 | 传真/文档压缩 |

### 2.2 TIFF 优势

- **灵活性**: 支持多种颜色空间和位深度
- **元数据**: 丰富的标签系统
- **多页**: 支持多页文档
- **无损**: 支持完全无损存储
- **专业**: 广泛用于专业领域

### 2.3 TIFF 限制

- **文件大小**: 无压缩时文件很大
- **复杂性**: 格式复杂，实现差异大
- **网页**: 浏览器不原生支持

---

## 三、为什么不提供 TIFF 压缩

本项目不提供 TIFF 压缩编码，原因如下：

1. **定位不同**: TIFF 主要用于存档和专业用途，不是网页图像格式
2. **压缩效率**: TIFF 的压缩效率不如 PNG、WebP、AVIF
3. **使用场景**: 需要 TIFF 的场景通常需要无损或特定压缩
4. **格式转换**: 建议将 TIFF 转换为更高效的格式（PNG、WebP、AVIF）

---

## 四、推荐工作流

```
TIFF 输入 → 解码 → 转换为其他格式
                    ├── PNG (无损)
                    ├── WebP (有损/无损)
                    ├── AVIF (有损/无损)
                    └── JPEG (有损)
```

---

## 五、依赖库

| 库 | 用途 | 特点 |
|----|------|------|
| `tiff` | TIFF 解码 | Rust 原生 TIFF 库 |
| `zune_image` | 图像处理 | 统一的图像接口 |

---

## 六、TIFF vs 其他格式

| 特性 | TIFF | PNG | WebP | AVIF |
|------|------|-----|------|------|
| 无损压缩 | ✅ | ✅ | ✅ | ✅ |
| 有损压缩 | ✅ | ❌ | ✅ | ✅ |
| 16位色深 | ✅ | ✅ | ❌ | ✅ |
| 32位浮点 | ✅ | ❌ | ❌ | ❌ |
| CMYK | ✅ | ❌ | ❌ | ❌ |
| 多页 | ✅ | ❌ | ❌ | ❌ |
| 网页支持 | ❌ | ✅ | ✅ | ✅ |
| 压缩效率 | 低 | 中 | 高 | 最高 |
