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
```

压缩可参考项目：https://github.com/Lymphatus/libcaesium/blob/c09613dbc85b39f525ba0398768e38e564eacf30/src/gif.rs

### webp

webp 封装了 libwebp-sys ：https://github.dev/jaredforth/webp
