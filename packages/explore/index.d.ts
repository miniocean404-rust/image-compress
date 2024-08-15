/* auto-generated by NAPI-RS */
/* eslint-disable */
export interface AppInfo {
  hwndId: number
  title: string
  bundleId: string
  isActive: boolean
  dir: string
  exec: string
  platform: Platform
}

export declare export declare function getOsFileManagerPath(): AppInfo

export declare export declare function getTargetTriple(): string

export declare export declare function initCustomTraceSubscriber(traceOutFilePath?: string | undefined | null): void

export declare const enum Platform {
  unknown = 'unknown',
  windows = 'windows',
  macos = 'macos'
}

