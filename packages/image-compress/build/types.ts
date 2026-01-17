import type { buildOptions } from "."

export type BuildOptions = typeof buildOptions & { cargoOptions?: string[] }
