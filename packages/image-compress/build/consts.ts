import path, { dirname } from "path"
import { fileURLToPath } from "url"

/**
 * @params 需要传入 import.meta.url
 */
export const __filename = (url: string) => fileURLToPath(url)

/**
 * @params 需要传入 import.meta.url
 */
export const __dirname = (url: string) => dirname(__filename(url))

export const cwd = process.cwd()
export const root = path.resolve(__dirname(import.meta.url), "../../../")

// 指向 bindings 工作区的 Cargo.toml
export const manifestPath = path.join(root, "bindings/Cargo.toml")
export const npmDir = path.join(cwd, "npm")
export const packageJsonPath = path.join(cwd, "package.json")
