n-api: https://juejin.cn/post/7322288075850039359?searchId=202402040121440A1FC55F67DF117FA08B
n-api: https://juejin.cn/post/7243413934765408315?searchId=202402040121440A1FC55F67DF117FA08B
n-api 使用：https://juejin.cn/post/7226879080415395897?searchId=202402040121440A1FC55F67DF117FA08B

# napi 构建命令

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
