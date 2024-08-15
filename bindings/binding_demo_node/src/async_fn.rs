use napi::{bindgen_prelude::AsyncTask, Env, JsNumber, Result, Task};
use napi_derive::napi;

pub struct AsyncFib {
    input: u32,
}

// 要返回一个异步的函数，我们需要实现 Task trait，
// 这个 trait 有两个关联类型，Output 和 JsValue，分别表示 Rust 函数的返回值类型和 JavaScript 中对应的类型。
// 在 compute 方法中，我们实现了具体的计算逻辑，而在 resolve 方法中，我们将计算结果转换成了 JavaScript 中的 JsNumber 类型。
// 然后我们在 async_fib 函数中，通过 AsyncTask::new 来创建一个异步任务，这个函数的返回值类型是 AsyncTask<AsyncFib>，
// 这个类型会被 napi-rs 自动转换成 JavaScript 中的 Promise 类型。
impl Task for AsyncFib {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        Ok(fib(self.input))
    }

    fn resolve(&mut self, env: Env, output: u32) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }
}

pub fn fib(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

// 指定 JS 侧的返回值类型为 Promise<number>
#[napi(ts_return_type = "Promise<string>")]
pub fn async_fib(input: u32) -> AsyncTask<AsyncFib> {
    AsyncTask::new(AsyncFib { input })
}
