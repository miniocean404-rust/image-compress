import { NapiCli } from "@napi-rs/cli"
import { createBuildCommand } from "@napi-rs/cli"
import { merge } from "es-toolkit"
import type { BuildOptions } from "./types"
import { defaultBuildOptions } from "./default-config"
import { npmDir, packageJsonPath } from "./consts"
import pkg from "../package.json"

const buildCommand = createBuildCommand(process.argv.slice(2))
export const buildOptions = buildCommand.getOptions()
const cli = new NapiCli()

async function main() {
  // 编译 Rust 代码为 Node.js 原生模块，支持交叉编译、类型定义生成等。
  const { task, abort } = await cli.build(
    merge<Partial<BuildOptions>, BuildOptions>(defaultBuildOptions, buildOptions),
  )
  const outputs = await task

  // 为不同平台创建独立的 npm 包目录结构
  await cli.createNpmDirs({
    npmDir,
    // 是否预演操作, 不写入文件
    dryRun: false,
  })

  // 从 GitHub Actions 或本地构建中收集 .node 文件到 npm 包中
  await cli.artifacts({
    packageJsonPath,
    // 只能是相对路径
    npmDir: "./npm",
    // 收集所有平台构建产物的中转目录, 只能是相对路径
    outputDir: "./",
    // 仅当目标包含 "wasm32-wasi-*" 时才需要构建输出dir的路径
    buildOutputDir: "./dist",
  })

  // 更新 package.json 并将产物复制到各平台包中
  await cli.prePublish({
    // package.json 路径
    packageJsonPath,

    // npm 包目录
    npmDir,

    // git 标签风格 (npm/lerna)
    tagStyle: "npm",

    // 创建 GitHub Release
    ghRelease: true,

    // GitHub Release 名称
    ghReleaseName: pkg.version,

    // 已存在的 GitHub Release ID
    ghReleaseId: undefined as string | undefined,

    // 跳过 optionalDependencies 包发布
    skipOptionalPublish: false,

    // 演练模式
    dryRun: true,
  })

  // 将主包的版本号同步到所有平台子包中。默认 createNpmDirs 就已经同步了
  await cli.version({
    npmDir,
    packageJsonPath,
  })

  for (const output of outputs) {
    if (output.kind !== "node") {
      console.log(output.path)
      // const { code } = await format(output.path, await readFileSync(output.path, 'utf-8'), oxfmtConfig as FormatOptions)
      // await writeFileSync(output.path, code)
    }
  }
}

main()
