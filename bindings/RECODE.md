# napi

### 构建命令

`"@napi-rs/cli": "^3.0.0-alpha.22"`版本命令：

1. --manifest-path 指定 workspace 文件夹路径
2. --platform 指定平台
3. -p 指定打包的 workspace 名称

- 将 .d.ts .js 文件都输出为 dist/node/binding 下，但是这时候不能指定 -o 为其他目录只能为 .

```shell
napi build  --manifest-path ./Cargo.toml --platform -p node --js ./dist/node/binding.js --dts ./dist/node/binding.d.ts -o .
```

- 将 .node .d.ts .js 文件都输出到 dist/node 下

```shell
napi build  --manifest-path ./Cargo.toml --platform -p node  -o dist/node
```

## 参考文章

n-api: https://juejin.cn/post/7322288075850039359?searchId=202402040121440A1FC55F67DF117FA08B
n-api: https://juejin.cn/post/7243413934765408315?searchId=202402040121440A1FC55F67DF117FA08B
n-api 使用：https://juejin.cn/post/7226879080415395897?searchId=202402040121440A1FC55F67DF117FA08B
如何基于 napi-rs 打造 Rust 前端工具链: https://juejin.cn/post/7243413934765408315?searchId=202308212000304BCD82BB562679DE002A

# wasm

### 命令

<!-- 创建新项目 -->

```shell
wasm-pack new hello-wasm
```

```shell
wasm-pack build --target nodejs
```

通过 --target 这个参数，我们就可以指定 wasm 模块在什么环境中使用：

选项 说明

- --target=web 编译为浏览器平台，使用了浏览器 esm 模块
- --target=nodejs 编译为 Nodejs 平台，使用了 commonjs 模块
- --target=bundler 编译为 esm 模块通用模块
- --target=no-modules 编译为 iife 模块, 可以通过 wasm_bindgen.xxx 来访问模块方法

### 测试 html

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Document</title>
</head>
<body>
  <script type="module">
    ;(async () => {
      const { default: init } = await import('./pkg/hello_wasm.js')
      const { fibonacci } = await init()
      console.time('wasm');
      console.log(fibonacci(40));
      console.timeEnd('wasm');
    })()
  </script>
</body>
</html>

```

## 参考文章

使用 Rust 封装一个 WASM NPM 包：https://juejin.cn/post/7219613068275449893?share_token=9fb9f439-c818-4885-9368-eeb829a89295
前端基建的未来？带你入门开发 Rust 前端工具链:https://juejin.cn/post/7270152997165432871?share_token=2437e3ac-00cc-408f-bb26-e00bee66ee96#heading-14
