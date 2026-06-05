# GIF 压缩原理

GIF (Graphics Interchange Format) 是一种支持动画和透明度的位图图像格式，使用 LZW 无损压缩算法。本项目使用 **gifsicle** 进行 GIF 压缩优化。

---

## 一、GIF 编码器 (gifsicle)

### 1.1 核心原理

gifsicle 是一个功能强大的 GIF 处理工具，支持：
- 有损和无损压缩
- 帧优化
- 颜色减少
- LZW 压缩优化

### 1.2 技术手段

#### 1.2.1 有损压缩 (Lossy)

```rust
pub lossy: u8,  // 0-200
```

| 有损级别 | 视觉效果 | 压缩效果 |
|----------|----------|----------|
| 0 | 无损 | 基准 |
| 20 | 几乎无损 | 减少 10-20% |
| 40 | 人眼难以察觉 | 减少 20-40%（默认） |
| 50 | 临界值 | 减少 30-50% |
| 80+ | 可见差异 | 减少 50%+ |

```rust
// 默认配置：lossy=40，人眼几乎无法察觉
GifOptions {
    lossy: 40,
    ...
}
```

有损压缩通过轻微修改像素值来提高 LZW 压缩效率。

#### 1.2.2 优化级别

```rust
pub optimize_level: u8,  // 1-3
```

| 级别 | 优化内容 | 速度 |
|------|----------|------|
| 1 | 基本优化 + 谨慎最小代码大小 | 最快 |
| 2 | 中等优化 + 积极清除 LZW 字典 | 中等 |
| 3 | 最大优化 + 收缩优化 + 积极清除 | 最慢 |

```rust
// 根据优化级别设置标志
let flags = match self.options.optimize_level {
    1 => GIF_WRITE_OPTIMIZE | GIF_WRITE_CAREFUL_MIN_CODE_SIZE,
    2 => GIF_WRITE_OPTIMIZE | GIF_WRITE_EAGER_CLEAR,
    _ => GIF_WRITE_OPTIMIZE | GIF_WRITE_SHRINK | GIF_WRITE_EAGER_CLEAR,
};
```

#### 1.2.3 LZW 压缩优化

GIF 使用 LZW (Lempel-Ziv-Welch) 无损压缩算法：

| 优化标志 | 说明 |
|----------|------|
| `GIF_WRITE_OPTIMIZE` | 启用 LZW 压缩优化 |
| `GIF_WRITE_CAREFUL_MIN_CODE_SIZE` | 使用更小的最小代码大小 |
| `GIF_WRITE_EAGER_CLEAR` | 更积极地清除 LZW 字典 |
| `GIF_WRITE_SHRINK` | 启用收缩优化（更激进） |

#### 1.2.4 颜色减少

```rust
pub reduce_colors: bool,
pub max_colors: u16,  // 2-256
```

GIF 最多支持 256 色调色板。减少颜色数量可以：
- 减小调色板大小
- 提高 LZW 压缩效率
- 可能导致色带效应

#### 1.2.5 帧优化

对于动画 GIF，gifsicle 会优化帧间差异：
- 只存储变化的像素
- 使用透明色表示不变区域
- 优化帧处置方法

### 1.3 编码流程

```rust
// 1. 首先将图像编码为标准 GIF 格式
let gif_data = self.encode_to_gif(image, width, height)?;

// 2. 然后使用 gifsicle 进行压缩
let compressed_data = self.compress_with_gifsicle(&gif_data)?;
```

### 1.4 gifsicle FFI 调用

```rust
unsafe fn compress_gif_file(&self, input_path: &str, output_path: &str) -> anyhow::Result<()> {
    // 读取 GIF 流
    let input_stream = gifsicle::Gif_ReadFile(input_file);
    
    // 设置压缩参数
    let gc_info = gifsicle::Gif_CompressInfo {
        flags,
        loss: self.options.lossy as c_int,
        padding,
    };
    
    // 写入压缩后的 GIF
    gifsicle::Gif_FullWriteFile(input_stream, &gc_info, output_file);
}
```

### 1.5 预设配置

```rust
impl GifOptions {
    /// 无损压缩
    pub fn lossless() -> Self {
        Self {
            lossy: 0,
            optimize_level: 3,
            reduce_colors: false,
            max_colors: 256,
        }
    }
    
    /// 有损压缩（根据质量）
    pub fn lossy(quality: u8) -> Self {
        // quality 100 -> lossy 0
        // quality 0 -> lossy 200
        let lossy = ((100 - quality.min(100)) as u16 * 2) as u8;
        Self { lossy, ... }
    }
    
    /// 最大压缩（人眼无感知）
    pub fn max_compression() -> Self {
        Self {
            lossy: 50,  // 人眼难以察觉的临界值
            optimize_level: 3,
            ...
        }
    }
}
```

### 1.6 默认配置

```rust
GifOptions {
    lossy: 40,           // 人眼几乎无法察觉
    optimize_level: 3,   // 最高优化级别
    reduce_colors: false,
    max_colors: 256,
}
```

### 1.7 代码位置

- 编码器: `crates/core/src/codecs/gif/encoder/gifsicle.rs`
- 解码器: `crates/core/src/codecs/gif/decoder.rs`
- 配置选项: `crates/core/src/codecs/gif/encoder/options.rs`

---

## 二、GIF 解码器

### 2.1 解码流程

```rust
impl<R: BufRead> GifDecoder<R> {
    pub fn try_new(source: R) -> Result<GifDecoder<R>, ImageErrors> {
        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        
        let decoder = options.read_info(source)?;
        // ...
    }
}
```

### 2.2 帧处理

GIF 动画的每一帧可能只包含部分图像：

```rust
// 获取帧的位置和尺寸
let frame_left = frame.left as usize;
let frame_top = frame.top as usize;
let frame_width = frame.width as usize;
let frame_height = frame.height as usize;

// 将帧数据复制到正确的位置
for y in 0..frame_height {
    for x in 0..frame_width {
        let dst_x = frame_left + x;
        let dst_y = frame_top + y;
        // 复制像素...
    }
}
```

---

## 三、GIF 格式特点

### 3.1 优势

- 广泛支持（所有浏览器）
- 支持动画
- 支持透明度（1位）
- 文件结构简单

### 3.2 限制

- 最多 256 色
- 只支持 1 位透明度（全透明或不透明）
- LZW 压缩效率有限
- 不支持半透明

---

## 四、GIF vs 其他动画格式

| 特性 | GIF | WebP 动画 | AVIF 动画 | APNG |
|------|-----|-----------|-----------|------|
| 颜色数 | 256 | 1600万 | 1600万 | 1600万 |
| 透明度 | 1位 | 8位 | 8位 | 8位 |
| 压缩效率 | 低 | 高 | 最高 | 中 |
| 浏览器支持 | 全部 | 广泛 | 广泛 | 广泛 |
| 文件大小 | 大 | 小 | 最小 | 中 |

---

## 五、有损压缩原理

gifsicle 的有损压缩通过以下方式工作：

1. **像素值修改**: 轻微调整像素值，使相邻像素更相似
2. **提高 LZW 效率**: 更多重复模式 = 更好的压缩
3. **视觉优化**: 在人眼不敏感的区域进行更多修改

```
原始: [100, 101, 102, 100, 99, 101]
有损: [100, 100, 100, 100, 100, 100]  // 更好的 LZW 压缩
```

---

## 六、依赖库

| 库 | 用途 | 特点 |
|----|------|------|
| `gifsicle` | GIF 压缩优化 | 有损/无损压缩、帧优化 |
| `gif` | GIF 编解码 | Rust 原生 GIF 库 |
| `libc` | FFI 支持 | 文件操作 |
