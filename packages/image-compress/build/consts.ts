import path from "path"

export const cwd = process.cwd()
export const root = path.resolve(__dirname, "../../../")

export const manifestPath = path.join(root, "Cargo.toml")
export const npmDir = path.join(cwd, "npm")
export const packageJsonPath = path.join(cwd, "package.json")
