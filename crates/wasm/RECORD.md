# 文章

https://juejin.cn/post/7156102433581383716#heading-9

# 打包

1. 构建

   ```shell
   # 目前仅支持 wasm32-unknown-unknown
   wasm-pack build
   ```

   构建 web

   ```shell
   wasm-pack build --release --target web

   # 可使用的命令：
   $env:CC="emcc.bat" ; $env:AR="emar.bat";wasm-pack  build ./crates/wasm --dev --out-dir ../../dist/wasm
   ```

2. 编译完成后，我们会发现根目录下多了一个 pkg/ 文件夹，里面就是我们的 WASM 产物所在的 npm 包了。
3. 包的入口文件是不带 \_bg 的 .js 文件，即 wasm_demo2.js。

# 平台

1. wasm32-unknown-unknown：实现 rust 到 wasm 的纯粹编译，不需要借助庞大的 C 库，因而产物体积更加小。通过内存分配器（wee_alloc）实现堆分配，从而可以使用我们想要的多种数据结构，例如 Map，List 等。利用 wasm-bindgen、web-sys/js-sys 实现与 js、ECMAScript、Web API 的交互。
2. wasm32-unknown-emscripten：首先需要了解 emscripten，借助 LLVM 轻松支持 rust 编译。目标产物通过 emscripten 提供标准库支持，保证目标产物可以完整运行，从而实现一个独立跨平台应用。
3. wasm32-wasi：主要是用来实现跨平台，通过 wasm 运行时实行跨平台模块通用，无特殊 web 属性

## emsdk

[emsdk](https://github.com/emscripten-core/emsdk)

## windows 安装方式

1. git clone https://github.com/emscripten-core/emsdk.git
2. cd emsdk
3. emsdk install --global latest
4. 激活：emsdk activate latest
5. 配置环境变量：emsdk_env.bat，需要手动配置环境变量-路径：`D:\soft-dev\env\emsdk\upstream\emscripten`
6. 测试：emcc -v
