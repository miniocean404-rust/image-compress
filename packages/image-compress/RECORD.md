# napi

1. cli 命令: https://napi.rs/docs/cli/build
2. cli 程序化 API: https://napi.rs/docs/cli/programmatic-api

## cli 命令参考

### 新建项目

文档: https://napi.rs/docs/cli/new

```sh
napi new <path> [--options]
```

### 基本用法

```bash
napi build [--options]
```

### 构建目标选项

- `--target, -t <string>`: 指定目标三元组，传递给 cargo build --target
- `--platform`: 在生成的绑定文件中添加平台三元组，如 `[name].linux-x64-gnu.node`
- `--release, -r`: 以 release 模式构建
- `--profile <string>`: 使用指定的配置文件构建

### 路径配置选项

- `--cwd <string>`: napi 命令执行的工作目录
- `--manifest-path <string>`: Cargo.toml 的路径
- `--config-path, -c <string>`: napi 配置 JSON 文件路径
- `--output-dir, -o <string>`: 构建文件输出目录，默认为 crate 文件夹
- `--target-dir <string>`: cargo 生成产物的目录

### JavaScript/TypeScript 生成选项

- `--js <string>`: 生成的 JS 绑定文件路径和文件名（需配合 --platform）
- `--no-js`: 禁用 JS 绑定文件生成
- `--esm`: 生成 ESM 格式而非 CJS 格式
- `--dts <string>`: 生成的类型定义文件路径
- `--const-enum`: 为 TypeScript 绑定生成 const enum
- `--js-package-name <string>`: 生成的 JS 绑定文件中的包名

### 交叉编译选项

- `--cross-compile, -x`: 实验性功能，使用 cargo-xwin (Windows) 或 cargo-zigbuild 交叉编译
- `--use-cross`: 使用 cross 而非 cargo
- `--use-napi-cross`: 使用 @napi-rs/cross-toolchain 交叉编译 Linux 目标

### 其他选项

- `--strip, -s`: 精简库文件以获得最小文件大小
- `--watch, -w`: 使用 cargo-watch 持续监视并构建
- `--features, -F <string[]>`: 激活的特性列表
- `--all-features`: 激活所有可用特性
- `--no-default-features`: 不激活默认特性
- `--verbose, -v`: 详细输出构建跟踪信息

### 传递标志给 Cargo

在 `--` 之后的标志会传递给 cargo build 命令：

```bash
napi build -- --locked
```

### 使用示例

```json
"scripts": {
  "build:napi-dev": "napi build --manifest-path ../../Cargo.toml --platform -p binding_explore --js ./index.js --dts ./index.d.ts --target-dir ../../target -o ./",
  "build:napi": "napi build --manifest-path ../../Cargo.toml --platform -p binding_explore --release -o ./",
  "prepack": "napi prepublish -p ./npm --tag-style npm --no-gh-release",
  "artifacts": "napi artifacts -d ../artifacts --npm-dir ./npm",
  "version": "napi version --npm-dir ./npm",
}
```

### --js-package-name 使用说明

当主包不在 npm scope 下，但平台包在 scope 下时（如避免 npm 垃圾检测），使用此选项覆盖包加载逻辑：

```bash
napi build --release --platform --js-package-name @napi-rs/snappy
```

这会将生成的 require 语句从 `require('snappy-darwin-x64')` 改为 `require('@napi-rs/snappy-darwin-x64')`。
