# crates 目录最佳实践问题清单

检查范围：`crates/` 下 Rust crate，包括 `core`、`image-compress`、`cli`、`utils`、`binding_node`、`binding_wasm`。

检查方式：结合 Rust Skills（`rust-router`、`coding-guidelines`、`m06-error-handling`、`m10-performance`、`m11-ecosystem`、`m15-anti-pattern`）规则，运行 `cargo metadata`、`cargo check --workspace --all-targets --all-features`、`cargo clippy --workspace --all-targets --all-features -- -D warnings`，并抽样核对高风险源码位置。

## Reasoning Chain

```text
+-- Layer 1: Error handling / unsafe / ownership signals
|   Problem: workspace 构建被缺失 bin 目标阻塞，源码中存在 unwrap/expect、unsafe、clone、调试输出等信号。
|       ^
+-- Layer 2: Rust best practices / ecosystem / performance
|   Constraint: library crate 应提供可恢复错误，FFI unsafe 要有明确安全边界，Cargo feature 不应声明不存在目标。
|   Rule: m06 建议生产与库代码优先 Result/?；m10 建议避免热路径不必要 clone/分配；m11 要求 workspace/features 可解析；m15 将无边界 unwrap、clone 和 unsafe 视为反模式信号。
|       v
+-- Layer 3: 图片压缩库约束
    Decision: 优先修复会阻塞 workspace 检查与可能破坏压缩稳定性的项，再清理性能和可维护性问题。
```

## Domain Constraints Analysis

- 本项目是图片压缩库，核心目标是“压缩后不得比原图更大”，因此编码路径的错误处理、临时文件安全、内存拷贝和状态统计都属于关键路径。
- `m11-ecosystem` 规则要求 Cargo workspace、feature 和依赖集成可解析；当前 `utils` 的 bin 目标不存在，导致 workspace 级检查无法继续。
- `m06-error-handling` 规则要求库代码对可预期失败返回 `Result`，而不是依赖 `unwrap()`/静默忽略错误。
- `m10-performance` 规则要求压缩热路径避免不必要分配和 clone；图片 buffer 通常较大，clone 成本不可忽略。
- `m15-anti-pattern` 规则将未说明安全边界的 `unsafe`、生产路径 `unwrap()`、无必要 clone、残留调试输出列为需要审查的反模式。

## 问题清单

### P0：workspace 级检查被 `utils` 缺失 bin 目标阻塞

- 位置：`crates/utils/Cargo.toml:7-10`
- 现象：`cargo check --workspace --all-targets --all-features` 和 `cargo clippy --workspace --all-targets --all-features -- -D warnings` 均失败。
- 证据：Cargo 报错 `can't find bin utils at path ... crates\utils\src\main.rs`。
- 原因：`[[bin]]` 声明了 `path = "./src/main.rs"`，但 `crates/utils/src/main.rs` 不存在；`--all-features` 会启用 `build-binary`，从而要求解析该 bin。
- 建议：二选一处理：
  - 如果 `utils` 不需要二进制入口，删除 `[[bin]]` 和 `build-binary` feature。
  - 如果确实需要二进制入口，补齐 `src/main.rs` 并明确其用途。
- 验证：重新运行 `cargo check --workspace --all-targets --all-features` 和 `cargo clippy --workspace --all-targets --all-features -- -D warnings`。

### P1：GIF 压缩临时文件名使用进程 ID，存在并发碰撞风险

- 位置：`crates/core/src/codecs/gif/encoder/gifsicle.rs:61-64`
- 现象：临时输入/输出路径为 `gifsicle_input_{pid}.gif` 和 `gifsicle_output_{pid}.gif`。
- 风险：同一进程内并发压缩多个 GIF 时会写入相同临时文件，可能导致数据串扰、输出损坏或清理掉其他任务的文件。
- 最佳实践依据：文件系统边界属于外部系统边界，不能假设单线程单任务；压缩库应支持并发调用或显式保护共享资源。
- 建议：使用唯一临时文件机制，例如 `tempfile::NamedTempFile`；若不新增依赖，则至少加入原子唯一后缀并保证 drop 清理。
- 验证：增加并发调用 `GifEncoder::encode_mem` 的测试，确认输出互不影响且临时文件被清理。

### P1：GIF FFI 路径的 `unsafe` 缺少显式安全边界说明与 RAII 清理

- 位置：`crates/core/src/codecs/gif/encoder/gifsicle.rs:81`、`101-158`
- 现象：直接调用 `libc::fopen/fclose` 和 `gifsicle::*` FFI，手动管理 `FILE*` 与 `Gif_Stream*`。
- 风险：早退路径、panic 或后续维护改动可能造成资源泄漏；`unsafe fn` 内部没有把不变量压缩到最小安全封装中。
- 最佳实践依据：Rust `unsafe` 应最小化作用域，并用安全封装表达所有权和释放责任；FFI 资源适合用 RAII guard。
- 建议：为 `FILE*`、`Gif_Stream*` 建立小型 Drop guard；将 FFI 调用集中在最小 `unsafe` 块，并写明安全前提。
- 验证：覆盖 `fopen` 失败、`Gif_ReadFile` 失败、`Gif_FullWriteFile` 失败路径，确认资源释放。

### P1：`ImageCompress::compress` 在空输入时可能产生无意义压缩率

- 位置：`crates/image-compress/src/compress.rs:171-175`
- 现象：`rate` 使用 `self.before_size as f64` 作为除数，但 `with_buffer` 可接收空 `Vec<u8>`。
- 风险：空输入时会产生 `NaN`/`inf` 类状态，且状态仍可能被设置为 `Done` 之前的中间值。
- 最佳实践依据：外部输入边界必须验证；图片压缩库应对非法输入返回明确错误。
- 建议：在 `compress` 开始时校验 `self.image.is_empty()` 或 `before_size == 0`，返回 `InvalidParameter`。
- 验证：增加空 buffer 测试，断言返回错误且 `state` 不进入 `Done`。

### P1：路径递归工具中 `to_str().unwrap()` 会在非 UTF-8 路径 panic

- 位置：`crates/utils/src/path/deep.rs:56`
- 现象：递归目录时把 `PathBuf` 转成 `&str` 并 `unwrap()`。
- 风险：Windows/Unix 都可能出现非 UTF-8 路径；工具库 panic 会影响调用方稳定性。
- 最佳实践依据：`m06-error-handling` 建议对外部文件系统失败返回 `Result`；`m15` 不建议库代码生产路径使用无保护 `unwrap()`。
- 建议：把内部 `deep_dir` 参数改为 `&Path`，避免来回转字符串；或者在转换失败时返回带上下文的错误。
- 验证：增加非 UTF-8 或转换失败路径的单元测试；至少确保 `cargo test -p utils --features fs` 通过。

### P1：目录遍历静默吞掉 IO 错误

- 位置：`crates/utils/src/path/deep.rs:21-27`、`49-56`
- 现象：`WalkDir` 和 `fs::read_dir` 的条目错误通过 `filter_map(Result::ok)` / `entry.ok()` / `metadata().ok()` 被忽略。
- 风险：权限错误、损坏路径或临时 IO 失败会被当成“没有文件”，调用方无法诊断。
- 最佳实践依据：文件系统是外部边界，错误不应静默丢失；如果要跳过，应显式记录或通过 API 表达跳过策略。
- 建议：保留错误并向上传播；若需要容错遍历，提供单独的“跳过错误”选项而不是默认吞错。
- 验证：构造不可读目录，确认调用方能拿到错误或可观测日志。

### P2：`LocalTimer` 每次格式化都 unwrap 常量 offset

- 位置：`crates/utils/src/log/time.rs:9-18`
- 现象：`east8()` 每次返回 `Option<FixedOffset>`，调用处 `east8().unwrap()`。
- 风险：当前 `8 * 60 * 60` 固定值理论上有效，但热路径日志格式化中重复构造和 unwrap 不必要。
- 最佳实践依据：`coding-guidelines` 建议保证值用 `expect()` 说明不变量；`m10` 建议减少热路径重复工作。
- 建议：改为 `static`/`LazyLock` 缓存固定 offset，或用 `expect("UTC+8 offset is valid")` 表达不变量。
- 验证：运行日志相关测试与 `cargo test -p utils --features log`。

### P2：`ImageCompress::compress` 返回时 clone 整个压缩结果

- 位置：`crates/image-compress/src/compress.rs:141-179`
- 现象：压缩结果先写入 `self.compressed_image`，返回时再 `clone()` 一份完整 buffer。
- 风险：大图压缩结果会产生额外内存分配和拷贝，影响吞吐和峰值内存。
- 最佳实践依据：`m10-performance` 建议热路径避免大对象 clone；`m15` 将无必要 clone 视为 ownership 设计信号。
- 建议：根据 API 语义选择一种所有权模型：
  - 若调用者只需要结果，返回 `Result<&[u8]>` 或提供 `compressed_image()` 访问器。
  - 若必须返回 `Vec<u8>`，考虑不在结构体内保留副本，或提供 `take_compressed_image()`。
- 验证：保持现有 API 消费方通过后，再用大图样例观察峰值内存。

### P2：GIF 编码路径存在额外 clone

- 位置：`crates/core/src/codecs/gif/encoder/gifsicle.rs:241-277`
- 现象：`RGBA` 分支先 `frame_data.clone()`，随后 `Frame::from_rgba_speed` 又对 `rgba_data.clone()` 取 `&mut`。
- 风险：GIF 帧数据可能很大，多余 clone 会放大内存和 CPU 成本。
- 最佳实践依据：`m10-performance` 建议图片 buffer 热路径避免不必要拷贝。
- 建议：让 `rgba_data` 成为唯一可变 buffer，直接传 `&mut rgba_data` 给 `Frame::from_rgba_speed`。
- 验证：运行 GIF 编码测试，比较输出是否一致。

### P2：`resize` 操作可能存在尺寸乘法溢出风险

- 位置：`crates/core/src/operations/resize.rs:40-58`
- 现象：`let new_length = dst_width * dst_height * image.depth().size_of();` 未使用 checked arithmetic。
- 风险：极大尺寸输入可能整数溢出，后续分配长度错误或 panic。
- 最佳实践依据：外部图片尺寸是边界输入；关键路径应对非法尺寸返回错误而非依赖溢出行为。
- 建议：使用 `checked_mul` 计算目标 buffer 长度，并在溢出时返回 `ImageOperationsErrors::GenericString`。
- 验证：增加超大目标尺寸测试。

### P2：`resize` native 分支每个 channel 创建线程，可能过度并行

- 位置：`crates/core/src/operations/resize.rs:52-110`
- 现象：`native` 下对每个 channel 使用 `std::thread::scope().spawn`。
- 风险：单张图片 channel 数较少时线程创建成本可能超过收益；上层若并发压缩多张图，会出现嵌套并行和调度开销。
- 最佳实践依据：`m10-performance` 强调先测量再优化；CPU-bound 并行应控制粒度。
- 建议：先 benchmark 单图/批量 resize；如确认有问题，改为复用线程池或串行处理小图。
- 验证：增加 resize benchmark，覆盖小图、大图、多图并发。

### P2：CLI 子命令存在未实现/调试输出痕迹

- 位置：`crates/cli/src/commands/png/imagequant.rs:43-56`、`crates/cli/src/commands/jpeg.rs:23`、`crates/cli/src/commands/png/oxipng.rs:102`
- 现象：`imagequant` 命令主体基本为空；若干命令只 `println!` 选项名。
- 风险：CLI 表面可用但实际不执行压缩，容易给用户造成成功错觉。
- 最佳实践依据：命令行入口应提供明确行为或返回未实现错误；不能静默成功。
- 建议：未实现命令返回 `anyhow::bail!("...")`，或补齐实际调用逻辑。
- 验证：运行 CLI 子命令，确认未实现路径非零退出或实际产出文件。

### P2：生产/库源码中有多个 `#[allow(...)]`，需要逐个确认是否仍必要

- 位置示例：`crates/binding_node/src/compress/options/webp.rs:5`、`crates/binding_node/src/compress/options/webp.rs:113`、`crates/core/src/codecs/jpeg/encoder/options.rs`、`crates/core/src/codecs/png/encoder/oxipng.rs`
- 现象：存在多处 `allow_attrs` 信号。
- 风险：长期保留 `allow` 会隐藏命名、死代码或 API 设计问题。
- 最佳实践依据：lint suppress 应局部、具体、有原因；不应作为常规规避手段。
- 建议：逐项核对：JS/NAPI 绑定为兼容外部命名可保留；内部 Rust 类型应尽量改成 idiomatic 命名并用转换层适配。
- 验证：移除不必要 allow 后运行 clippy。

### P3：测试中大量 `dbg!`/`println!`/`unwrap()` 影响信噪比

- 位置示例：`crates/core/tests/png.rs`、`crates/core/tests/jpeg.rs`、`crates/core/tests/webp.rs`、`crates/core/tests/avif.rs`。
- 现象：测试文件中存在大量调试输出和 `unwrap()`。
- 风险：测试日志噪声高，失败时定位成本增加；`unwrap()` 的 panic 信息不如 `?`/`expect` 清晰。
- 最佳实践依据：`m06` 允许测试中使用 `unwrap()`，但更推荐 `expect()` 或返回 `Result` 以保留上下文。
- 建议：测试函数统一返回 `anyhow::Result<()>` 或使用 `expect("具体上下文")`；调试输出只在必要时用 `-- --nocapture` 或 tracing 控制。
- 验证：运行 `cargo test --workspace` 并确认输出可读。

### P3：依赖和 feature 默认面过大，可能影响构建体积与可移植性

- 位置：`crates/core/Cargo.toml:6-25`、`Cargo.toml:61-63`、`crates/binding_node/Cargo.toml:25-33`
- 现象：`image_compress_core` 默认启用 `wasm`、`operations`、`native`；workspace `tokio` 使用 `features = ["full"]`；`napi-derive` 直接依赖 GitHub `main` 分支。
- 风险：默认构建牵引 AVIF/GIF/JPEG/WebP 等重依赖，增加环境要求和 CI 时间；Git 分支依赖不可复现；`tokio/full` 会放大依赖面。
- 最佳实践依据：`m11-ecosystem` 建议 feature 只启用需要的能力，并避免不稳定依赖来源。
- 建议：
  - 将核心库 default feature 调窄，按 native/wasm 场景分别启用。
  - `tokio` 按实际模块使用精确 feature。
  - `napi-derive` 优先使用 crates.io 固定版本；若必须用 git，固定 commit。
- 验证：分别运行 native、wasm、node 构建命令，确认 feature 组合清晰可复现。

## Recommended Solution

建议按以下顺序处理：

1. 先修复 `crates/utils/Cargo.toml` 的缺失 bin 目标，让 workspace check/clippy 能恢复工作。
2. 再处理 GIF 临时文件并发碰撞和 FFI 资源管理，因为这是压缩关键路径且影响正确性。
3. 补齐外部输入边界校验：空 buffer、非 UTF-8 路径、目录遍历错误、resize 尺寸溢出。
4. 之后再做性能清理：移除大 buffer clone、评估 resize 并行粒度。
5. 最后清理 CLI 未实现路径、测试日志噪声、`allow` 和默认 feature 面。

## 后续验证命令

```shell
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
