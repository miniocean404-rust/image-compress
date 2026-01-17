import { manifestPath, packageJsonPath } from "./consts"
import dedent from "dedent"

export const defaultBuildOptions = {
  // 构建目标三元组，传递给 `cargo build --target`
  // target: undefined,

  // napi 命令执行的工作目录，所有其他路径选项都相对于此路径
  cwd: process.cwd(),

  // `Cargo.toml` 文件路径
  manifestPath,

  // `napi` 配置 JSON 文件路径
  // configPath: undefined,

  // `package.json` 文件路径
  packageJsonPath,

  // 所有 crate 生成产物的目录，参见 `cargo build --target-dir`
  // targetDir: undefined,

  // 所有构建文件的输出路径，默认为 crate 文件夹
  outputDir: "./",

  // 在生成的 Node.js 绑定文件中添加平台三元组，例如：`[name].linux-x64-gnu.node`
  platform: true,

  // 生成的 JS 绑定文件中的包名，仅在使用 `--platform` 标志时有效
  jsPackageName: "@api/explore",

  // 是否为 TypeScript 绑定生成 const enum
  constEnum: true,

  // 生成的 JS 绑定文件的路径和文件名，仅在使用 `--platform` 标志时有效，相对于 `--output-dir`
  jsBinding: "index.js",

  // 是否禁用 JS 绑定文件的生成，仅在使用 `--platform` 标志时有效
  noJsBinding: false,

  // 生成的类型定义文件的路径和文件名，相对于 `--output-dir`
  dts: "index.d.ts",

  // 生成的类型定义文件的自定义文件头，仅在启用 `typedef` 功能时有效
  dtsHeader: dedent`
    // Type definitions for explore
    // Project: https://github.com/giegie/explore
    // Definitions by: Giegie <https://github.com/giegie>
    // Definitions: https://github.com/DefinitelyTyped/DefinitelyTyped\n\n
  `,

  // 是否禁用生成的类型定义文件的默认文件头，仅在启用 `typedef` 功能时有效
  noDtsHeader: false,

  // 是否启用类型定义文件缓存，默认为 true
  dtsCache: false,

  // 是否生成 ESM 格式的 JS 绑定文件而非 CJS 格式，仅在使用 `--platform` 标志时有效
  esm: true,

  // 是否精简库文件以获得最小文件大小
  strip: true,

  // 以 release 模式构建
  release: false,

  // 详细输出构建命令跟踪信息
  verbose: true,

  // 仅构建指定的二进制文件
  bin: undefined,

  // 构建指定的库或当前工作目录中的库
  package: "binding_explore",

  // 使用指定的配置文件构建产物
  // profile: undefined,

  // [实验性] 在 Windows 上使用 `cargo-xwin`，在其他平台上使用 `cargo-zigbuild` 进行交叉编译
  // crossCompile: undefined,

  // [实验性] 使用 cross 而非 `cargo`
  useCross: false,

  // [实验性] 使用 @napi-rs/cross-toolchain 交叉编译 Linux arm/arm64/x64 gnu 目标
  useNapiCross: false,

  // 使用 `cargo-watch` crate 监视 crate 变化并持续构建
  watch: false,

  // 要激活的特性列表（以空格分隔）
  features: undefined,

  // 激活所有可用特性
  allFeatures: undefined,

  // 不激活 `default` 特性
  noDefaultFeatures: false,

  // 额外的 cargo 命令参数
  cargoOptions: [],
}
