# crates core 包目标

用户给了一张图片，然后将这张图在保留清晰度的情况下尽可能压缩，并且不能在压缩图片后还比原图要更大

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust-based image compression library with Node.js (via napi-rs) and WebAssembly bindings. Supports PNG, JPEG, WebP, AVIF, GIF, and TIFF formats.

## Build Commands

### Rust Core Library

```shell
# Build default workspace (image-compress)
cargo build

# Build with release optimizations
cargo build --release

# Run a specific package binary
cargo r --package explore --bin <bin_name>

# Build core with native features (jpeg, webp, gif, tiff, avif)
cargo build -p image_compress_core --features native

# Build core with wasm features
cargo build -p image_compress_core --features wasm
```

### Node.js Bindings (napi-rs)

```shell
# From packages/image-compress or packages/explore:
pnpm run build:node-dev   # Development build
pnpm run build:node       # Release build

# Workspace-level build
pnpm run build:node       # Builds all @giegie/* packages
```

### WebAssembly Bindings

```shell
# From packages/image-compress:
pnpm run build:wasm-dev   # Development build
pnpm run build:wasm       # Release build

# Workspace-level
pnpm run build:wasm
```

### Testing

```shell
# Node.js binding tests
pnpm run test:node        # From packages/image-compress or packages/explore
```

## Architecture

### Workspace Structure

**Root workspace** (`/Cargo.toml`): Contains core Rust crates

- `crates/core` - Core compression library (`image_compress_core`)
- `crates/image-compress` - High-level compression API (`image_compress`)
- `crates/cli` - CLI tool (`image_compress_cli`)
- `crates/explore` - System exploration utilities
- `crates/utils` - Shared utilities

**Bindings workspace** (`/bindings/Cargo.toml`): Contains FFI bindings

- `binding_compress_node` - Node.js bindings via napi-rs
- `binding_compress_wasm` - WebAssembly bindings via wasm-bindgen
- `binding_explore_node` - Node.js bindings for explore
- `binding_demo_node` - Demo/example bindings

**NPM packages** (`/packages/`):

- `@giegie/image-compress` - Published npm package
- `@giegie/explore` - Published npm package

### Core Library Features (crates/core)

Feature flags control codec and operation inclusion:

**Codec features:**

- `png` - PNG support (oxipng, imagequant, lodepng)
- `jpeg` - JPEG support (mozjpeg)
- `webp` - WebP support (libwebp)
- `avif` - AVIF support (ravif, libavif)
- `gif` - GIF support (gifsicle)
- `tiff` - TIFF support (decode only)

**Operation features:**

- `transform` - Format conversion
- `resize` - Image resizing (fast_image_resize)
- `icc` - ICC color profile handling (lcms2)
- `quantize` - Color quantization

**Preset features:**

- `wasm` - Enables mem + png (for WebAssembly builds)
- `native` - Enables jpeg, webp, gif, tiff, avif (for native builds)
- `operations` - Enables transform, resize, icc, quantize

### Build Environment Requirements

- **oxipng wasm**: Requires LLVM clang
- **ravif (AVIF)**: Requires nasm assembler; Windows also needs Perl

## NPM Publishing

```shell
pnpm run publish  # Publishes @giegie/explore and @giegie/image-compress
```
