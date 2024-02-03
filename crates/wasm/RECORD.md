# 文章

https://juejin.cn/post/7156102433581383716#heading-9

# 打包

1. 构建

   ```shell
   wasm-pack build
   ```

   构建 web

   ```shell
   wasm-pack build --release --target web
   ```

2. 编译完成后，我们会发现根目录下多了一个 pkg/ 文件夹，里面就是我们的 WASM 产物所在的 npm 包了。
3. 包的入口文件是不带 \_bg 的 .js 文件，即 wasm_demo2.js。
