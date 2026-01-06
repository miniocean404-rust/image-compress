# 未完成 TODO

1. bak 下 与 Rimage 差异-20260105 没有查看修改，暂时搁置

# 命令

## workspack 命令

```shell
cargo r --package explore --bin build_bin_name

# 编译所有包 -p = --package
cargo build -p explore -p utils -p image_compress_core -p image_compress -p cli

# 编译整个 workspace（所有包）
cargo build --workspace
# 或简写
cargo build --all

# 运行测试
# 其他参数:
# -- --nocapture：实时输出，测试运行时就能看到
# -- --show-output：测试完成后统一显示输出
cargo test -p image_compress_core --test avif --features avif -- encode_mem_avif --exact --nocapture
```

压缩可参考项目：https://github.com/Lymphatus/libcaesium/blob/c09613dbc85b39f525ba0398768e38e564eacf30/src/gif.rs

### webp

webp 封装了 libwebp-sys ：https://github.dev/jaredforth/webp
