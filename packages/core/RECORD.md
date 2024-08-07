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
