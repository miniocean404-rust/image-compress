# NAPI-RS 开发助手

使用 Rust 构建高性能 Node.js 原生扩展的专家助手。提供项目初始化、导出机制、类定义、异步函数、多线程回调等全方位支持。

**触发场景:**
- 创建或修改 NAPI-RS 项目
- 使用 `#[napi]` 宏导出函数、类、常量
- 实现异步函数和 Tokio 集成
- 处理 ThreadsafeFunction 多线程回调
- 配置跨平台构建和发布

**关键词:** napi-rs, napi, node addon, rust binding, #[napi], ThreadsafeFunction, tokio async, node.js native, cross-platform build, napi-derive

---

## 核心知识

### 1. 项目初始化

#### 使用 CLI 创建项目

```bash
# 安装 CLI
npm install -g @napi-rs/cli

# 创建新项目
napi new
```

交互式提示：
- Package name - 包名
- Target platforms - 目标平台
- GitHub actions - CI/CD 配置

#### 依赖配置

```toml
[dependencies]
napi = { version = "3", features = ["async"] }
napi-derive = "3"

[build-dependencies]
napi-build = "2"
```

**特性标志:**
- `async` - 异步函数支持
- `tokio_rt` - Tokio 运行时集成
- `napi4` ~ `napi8` - Node-API 版本特性

---

### 2. 导出机制

#### 导出函数

```rust
use napi_derive::napi;

#[napi]
pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}
```

生成 TypeScript:
```typescript
export function sum(a: number, b: number): number
```

#### 导出常量

```rust
#[napi]
pub const DEFAULT_COST: u32 = 12;
```

#### 导出枚举

```rust
#[napi]
pub enum Status {
    Pending,
    Running,
    Completed,
}
```

#### 访问 exports 对象

```rust
use napi::bindgen_prelude::*;

#[napi(module_exports)]
pub fn exports(mut export: Object) -> Result<()> {
    let symbol = Symbol::for_desc("NAPI_RS_SYMBOL");
    export.set_named_property("NAPI_RS_SYMBOL", symbol)?;
    Ok(())
}
```

---

### 3. 类定义

#### 基本类

```rust
#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
    engine: QueryEngine,
}
```

#### 构造函数

**默认构造函数** - 所有字段 `pub`:

```rust
#[napi(constructor)]
pub struct Animal {
    pub name: String,
    pub kind: u32,
}
```

**自定义构造函数**:

```rust
#[napi]
impl JsQueryEngine {
    #[napi(constructor)]
    pub fn new() -> Self {
        JsQueryEngine {
            engine: QueryEngine::new()
        }
    }
}
```

**工厂方法**:

```rust
#[napi(factory)]
pub fn with_initial_count(count: u32) -> Self {
    JsQueryEngine {
        engine: QueryEngine::with_initial_count(count)
    }
}
```

#### 类方法

```rust
#[napi]
impl Database {
    #[napi]
    pub async fn query(&self, sql: String) -> Result<String> {
        // 执行查询
        Ok(format!("Result: {}", sql))
    }
}
```

#### Getter 和 Setter

```rust
#[napi(getter)]
pub fn status(&self) -> napi::Result<u32> {
    Ok(self.engine.status())
}

#[napi(setter)]
pub fn count(&mut self, count: u32) {
    self.engine.count = count;
}
```

#### 属性控制

```rust
#[napi(writable = false, enumerable = true, configurable = false)]
pub fn get_num(&self) -> i32 {
    self.num
}
```

#### 类作为参数

类所有权转移到 JavaScript，只能通过引用传递：

```rust
pub fn accept_class(engine: &QueryEngine) {}
pub fn accept_class_mut(engine: &mut QueryEngine) {}
```

#### 自定义终结逻辑

```rust
use napi::bindgen_prelude::*;

#[napi(custom_finalize)]
pub struct CustomFinalize {
    inner: Vec<u8>,
}

impl ObjectFinalize for CustomFinalize {
    fn finalize(self, mut env: Env) -> Result<()> {
        env.adjust_external_memory(-(self.inner.len() as i64))?;
        Ok(())
    }
}
```

---

### 4. 异步函数

#### Tokio 集成

```rust
use napi::bindgen_prelude::*;
use tokio::fs;

#[napi]
pub async fn read_file_async(path: String) -> Result<Buffer> {
    let content = fs::read(path).await?;
    Ok(content.into())
}
```

生成 TypeScript:
```typescript
export function readFileAsync(path: string): Promise<Buffer>
```

#### 使用 `&mut self` 的安全性

异步函数中使用 `&mut self` 需要 `unsafe`:

```rust
#[napi]
impl Engine {
    #[napi]
    pub async unsafe fn run(&mut self) {}
}
```

**原因:** `self` 同时被 Node.js 运行时拥有。

#### 自动引用机制

以下参数类型会自动转换为 `Reference`:
- `&self`
- `&mut self`
- `This<T>`

系统在异步调用前调用 `napi_create_reference`，调用后调用 `napi_delete_reference`，防止 GC 回收。

---

### 5. Object 数据传递

`Object` 用于将 Rust 结构体转换为 JavaScript 对象，不能分配方法。

#### 定义 Object

```rust
#[napi(object)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u32,
}
```

**要求:**
- 所有字段必须 `pub`
- `impl` 块不影响 JavaScript Object
- 可作为函数参数或返回类型

#### 使用示例

```rust
#[napi]
pub fn create_server(config: Config) -> Result<String> {
    Ok(format!("Server at {}:{}", config.host, config.port))
}

#[napi]
pub fn get_default_config() -> Config {
    Config {
        host: "localhost".to_string(),
        port: 3000,
        timeout: 5000,
    }
}
```

#### 克隆行为

⚠️ **重要:** JavaScript Object 会被克隆：
- JavaScript 修改不影响 Rust
- Rust 修改不影响 JavaScript

---

### 6. ThreadsafeFunction

从其他线程调用 JavaScript 函数。

#### 基本使用

```rust
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::thread;

#[napi]
pub fn process_in_background(callback: ThreadsafeFunction<String>) {
    thread::spawn(move || {
        for i in 0..10 {
            let message = format!("Processing step {}", i);
            callback.call(Ok(message), ThreadsafeFunctionCallMode::NonBlocking);
        }
    });
}
```

生成 TypeScript:
```typescript
export function processInBackground(
    callback: (err: null | Error, result: string) => void
): void
```

#### 返回值捕获

```rust
ThreadsafeFunction<u32, u32>  // 输入类型, 返回类型

callback.call_with_return_value(
    Ok(1),
    ThreadsafeFunctionCallMode::Blocking,
    |ret, _| {
        println!("返回值: {:?}", ret);
        Ok(())
    }
);
```

#### 错误处理策略

**CalleeHandled: true (默认)** - Node.js 约定:
```rust
ThreadsafeFunction<u32, u32, u32, Status, true>
// JavaScript: (err: Error | null, result: number) => void
```

**CalleeHandled: false** - 直接传值:
```rust
ThreadsafeFunction<u32, (), u32, Status, false>
// JavaScript: (arg: number) => void
```

#### 弱引用

第 6 个泛型参数设为 `true` 避免保持事件循环活动。

#### 队列大小控制

第 7 个参数限制队列大小。`NonBlocking` 模式下队列满时返回 `Status::QueueFull`。

---

## 常见模式

### 模式 1: 同步函数导出

```rust
#[napi]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 模式 2: 异步文件操作

```rust
use napi::bindgen_prelude::*;
use tokio::fs;

#[napi]
pub async fn read_file(path: String) -> Result<String> {
    let content = fs::read_to_string(path)
        .await
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(content)
}
```

### 模式 3: 类与方法

```rust
#[napi]
pub struct Database {
    connection: String,
}

#[napi]
impl Database {
    #[napi(constructor)]
    pub fn new(connection: String) -> Self {
        Database { connection }
    }

    #[napi]
    pub async fn query(&self, sql: String) -> Result<String> {
        Ok(format!("Result for: {}", sql))
    }
}
```

### 模式 4: 线程安全回调

```rust
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use std::thread;

#[napi]
pub fn process_in_background(callback: ThreadsafeFunction<String>) {
    thread::spawn(move || {
        for i in 0..10 {
            let message = format!("Processing step {}", i);
            callback.call(Ok(message), ThreadsafeFunctionCallMode::NonBlocking);
        }
    });
}
```

### 模式 5: Object 数据传递

```rust
#[napi(object)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u32,
}

#[napi]
pub fn create_server(config: Config) -> Result<String> {
    Ok(format!("Server at {}:{}", config.host, config.port))
}
```

---

## 构建和发布

### 构建命令

```bash
# 开发构建
napi build

# 发布构建
napi build --release

# 指定目标平台
napi build --platform --release
```

### 包分发策略

NAPI-RS 使用多包方法。例如 `@cool/core`:

- `@cool/core` - 主 JavaScript 包
- `@cool/core-darwin-x64` - macOS 二进制
- `@cool/core-win32-x64` - Windows 二进制
- `@cool/core-linux-arm64-gnu` - Linux ARM 二进制

每个平台包在 package.json 中包含 `"os"` 和 `"cpu"` 字段，作为主包的 `optionalDependencies`。

### 发布到 NPM

```bash
# 构建所有平台
napi build --release --platform

# 发布
npm publish
```

---

## 最佳实践

### 1. 错误处理

始终使用 `Result<T>`:

```rust
#[napi]
pub fn risky_operation() -> Result<String> {
    let result = some_fallible_operation()?;
    Ok(result)
}
```

### 2. 类型安全

利用 Rust 类型系统:

```rust
#[napi]
pub enum Status {
    Pending,
    Running,
    Completed,
}
```

### 3. 生命周期管理

长期存在的对象使用类而非 Object:

```rust
#[napi]
pub struct LongLivedResource {
    // 内部状态
}

#[napi]
impl LongLivedResource {
    #[napi(constructor)]
    pub fn new() -> Self {
        LongLivedResource {}
    }
}
```

### 4. 性能优化

- 大数据传输使用 `Buffer` 而非 `Vec<u8>`
- 避免不必要的克隆和分配
- I/O 密集型操作使用异步函数

### 5. 跨平台兼容性

- 测试所有目标平台
- 使用条件编译处理平台特定代码
- 遵循 NAPI-RS 平台支持矩阵

---

## 支持的平台

**操作系统:**
- Windows
- macOS
- Linux (glibc)
- Linux (musl)
- FreeBSD
- Android

**架构:**
- i686, x64, aarch64, arm, riscv64, s390x, ppc64le

**Node.js 版本:** Node 10-22 (官方测试 Node 12+)

---

## 资源链接

- **官方网站:** https://napi.rs
- **GitHub:** https://github.com/napi-rs/napi-rs
- **文档:** https://napi.rs/docs
- **Discord:** https://discord.gg/napi-rs

---

## 工作流程

当用户请求 NAPI-RS 相关任务时：

1. **识别场景** - 确定是初始化、导出、类定义、异步还是多线程
2. **检查依赖** - 验证 Cargo.toml 配置和特性标志
3. **提供代码** - 使用上述模式生成符合最佳实践的代码
4. **类型定义** - 说明生成的 TypeScript 类型
5. **构建指导** - 提供构建和测试命令

**注意事项:**
- 异步函数中 `&mut self` 需要 `unsafe`
- Object 会被克隆，修改不互相影响
- ThreadsafeFunction 需要正确处理生命周期
- 类的所有权转移到 JavaScript，只能通过引用传递
