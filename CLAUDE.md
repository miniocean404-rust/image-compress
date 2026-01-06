# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

基于 Rust 的图片压缩库，提供 Node.js (通过 napi-rs) 和 WebAssembly 绑定。支持 PNG、JPEG、WebP、AVIF、GIF 和 TIFF 格式。

## 构建命令

### Rust 核心库

```shell
# 构建默认工作区 (image-compress)
cargo build

# 构建 release 优化版本
cargo build --release

# 运行指定包的二进制文件
cargo r --package explore --bin <bin_name>

# 构建带 native 特性的核心库 (jpeg, webp, gif, tiff, avif)
cargo build -p image_compress_core --features native

# 构建带 wasm 特性的核心库
cargo build -p image_compress_core --features wasm
```

### Node.js 绑定 (napi-rs)

```shell
# 在 packages/image-compress 或 packages/explore 目录下:
pnpm run build:node-dev   # 开发构建
pnpm run build:node       # 发布构建

# 工作区级别构建
pnpm run build:node       # 构建所有 @giegie/* 包
```

### WebAssembly 绑定

```shell
# 在 packages/image-compress 目录下:
pnpm run build:wasm-dev   # 开发构建
pnpm run build:wasm       # 发布构建

# 工作区级别
pnpm run build:wasm
```

### 测试

```shell
# Node.js 绑定测试
pnpm run test:node        # 在 packages/image-compress 或 packages/explore 目录下
```

## 架构

### 工作区结构

**根工作区** (`/Cargo.toml`): 包含核心 Rust crates

- `crates/core` - 核心压缩库 (`image_compress_core`)
- `crates/image-compress` - 高级压缩 API (`image_compress`)
- `crates/cli` - 命令行工具 (`image_compress_cli`)
- `crates/explore` - 系统探索工具
- `crates/utils` - 共享工具库

**绑定工作区** (`/bindings/Cargo.toml`): 包含 FFI 绑定

- `binding_compress_node` - 通过 napi-rs 的 Node.js 绑定
- `binding_compress_wasm` - 通过 wasm-bindgen 的 WebAssembly 绑定
- `binding_explore_node` - explore 的 Node.js 绑定
- `binding_demo_node` - 演示/示例绑定

**NPM 包** (`/packages/`):

- `@giegie/image-compress` - 已发布的 npm 包
- `@giegie/explore` - 已发布的 npm 包

### 核心库特性 (crates/core)

特性标志控制编解码器和操作的包含:

**编解码器特性:**

- `png` - PNG 支持 (oxipng, imagequant, lodepng)
- `jpeg` - JPEG 支持 (mozjpeg)
- `webp` - WebP 支持 (libwebp)
- `avif` - AVIF 支持 (ravif, libavif)
- `gif` - GIF 支持 (gifsicle)
- `tiff` - TIFF 支持 (仅解码)

**操作特性:**

- `transform` - 格式转换
- `resize` - 图片缩放 (fast_image_resize)
- `icc` - ICC 色彩配置文件处理 (lcms2)
- `quantize` - 颜色量化

**预设特性:**

- `wasm` - 启用 mem + png (用于 WebAssembly 构建)
- `native` - 启用 jpeg, webp, gif, tiff, avif (用于原生构建)
- `operations` - 启用 transform, resize, icc, quantize

### 构建环境要求

- **oxipng wasm**: 需要 LLVM clang
- **ravif (AVIF)**: 需要 nasm 汇编器; Windows 还需要 Perl

## NPM 发布

```shell
pnpm run publish  # 发布 @giegie/explore 和 @giegie/image-compress
```

# crates core 实现目标

用户给了一张图片，然后将这张图在保留清晰度的情况下尽可能压缩，并且不能在压缩图片后还比原图要更大

# 命令

所有 node 包命令相关使用 pnpm
