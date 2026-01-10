# CLAUDE.md

本文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

---

## AI 助手行为规范 (Claude Protocol)

你是一名遵循最高软件工程标准的资深程序员。在本次会话中，你必须严格遵守以下所有规则。

### 1. 核心原则

- **遵循架构**: 你不得偏离项目已定义的架构决策和代码模式。
- **单一职责**: 你生成的每个代码单元都应只解决一个明确的问题。
- **禁止猜测**: 你的工作范围严格限定于用户提供的"当前任务"。如果任务描述不清晰，你必须提问而不是做出假设。

### 2. 环境与依赖

- **严格遵循版本**: 本项目的所有环境和依赖都已锁定。你不得提出任何与已定义版本不符的代码或建议。
- **不引入新依赖**: 未经用户明确许可，你不得引入任何新的第三方库。

### 3. 工作流程

- **任务驱动**: 你的唯一目标是完成用户本次会话提供的那个独立任务。
- **先检查后编码**: 如用户要求，你的第一步是提供环境检查命令，等待用户确认环境就绪。

### 4. 代码质量

- **遵循规范**: 你产出的所有代码都必须遵循项目已配置的 Linter 和 Formatter 规则。
- **健壮性**: 在适当的地方添加错误处理和边界条件检查。
- **可读性**: 对复杂逻辑添加简洁的注释，并使用清晰的命名。

---

## 项目概述

基于 Rust 的图片压缩库，提供 Node.js (通过 napi-rs) 和 WebAssembly 绑定。支持 PNG、JPEG、WebP、AVIF、GIF 和 TIFF 格式。

**核心目标**: 在保留图片清晰度的前提下尽可能压缩，且压缩后不得比原图更大。

## 常用命令

| 类型            | 命令                                |
| --------------- | ----------------------------------- |
| Node 包管理     | `pnpm`                              |
| Rust 文档       | `cargo doc --open`                  |
| Crates 在线文档 | https://docs.rs/crate/{库名}/{版本} |

## 构建命令

### Rust 核心库

```shell
cargo build                                        # 构建默认工作区
cargo build --release                              # 发布构建
cargo build -p image_compress_core --features native  # 原生特性 (jpeg, webp, gif, tiff, avif)
cargo build -p image_compress_core --features wasm    # WASM 特性
cargo r --package explore --bin <bin_name>         # 运行指定二进制
```

### Node.js / WebAssembly 绑定

```shell
pnpm run build:node-dev   # Node.js 开发构建
pnpm run build:node       # Node.js 发布构建
pnpm run build:wasm-dev   # WASM 开发构建
pnpm run build:wasm       # WASM 发布构建
pnpm run test:node        # 运行测试
pnpm run publish          # 发布 NPM 包
```

## 架构

### 工作区结构

**根工作区** (`/Cargo.toml`)

| Crate                   | 说明                               |
| ----------------------- | ---------------------------------- |
| `crates/core`           | 核心压缩库 (`image_compress_core`) |
| `crates/image-compress` | 高级压缩 API (`image_compress`)    |
| `crates/cli`            | 命令行工具                         |
| `crates/explore`        | 系统探索工具                       |
| `crates/utils`          | 共享工具库                         |

**绑定工作区** (`/bindings/Cargo.toml`)

| Binding                 | 说明                            |
| ----------------------- | ------------------------------- |
| `binding_compress_node` | Node.js 绑定 (napi-rs)          |
| `binding_compress_wasm` | WebAssembly 绑定 (wasm-bindgen) |
| `binding_explore_node`  | explore Node.js 绑定            |
| `binding_demo_node`     | 演示/示例绑定                   |

**NPM 包** (`/packages/`): `@giegie/image-compress`, `@giegie/explore`

### 核心库特性 (crates/core)

| 类别     | 特性         | 说明                             |
| -------- | ------------ | -------------------------------- |
| 编解码器 | `png`        | oxipng, imagequant, lodepng      |
|          | `jpeg`       | mozjpeg                          |
|          | `webp`       | libwebp                          |
|          | `avif`       | ravif, libavif                   |
|          | `gif`        | gifsicle                         |
|          | `tiff`       | 仅解码                           |
| 操作     | `transform`  | 格式转换                         |
|          | `resize`     | 图片缩放 (fast_image_resize)     |
|          | `icc`        | ICC 色彩配置文件 (lcms2)         |
|          | `quantize`   | 颜色量化                         |
| 预设     | `wasm`       | mem + png                        |
|          | `native`     | jpeg, webp, gif, tiff, avif      |
|          | `operations` | transform, resize, icc, quantize |

### 构建环境要求

- **oxipng wasm**: 需要 LLVM clang
- **ravif (AVIF)**: 需要 nasm 汇编器; Windows 还需要 Perl
