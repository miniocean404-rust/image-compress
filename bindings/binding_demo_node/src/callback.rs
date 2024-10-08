use std::thread;

use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;

/// ## 调用后结果
/// \[0\] \[1\] ..
///
/// ts_args_type: 强制指定参数类型
#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
    let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
        // ctx.value 即 Rust 调用 JS 函数时传递的入参，封装成 Vec 传递给 JS 函数
        .create_threadsafe_function(0, |ctx| ctx.env.create_uint32(ctx.value).map(|v| vec![v]))?;

    for n in 0..100 {
        let tsfn = tsfn.clone();
        thread::spawn(move || {
            // 通过 tsfn.call 来调用 JS 函数
            tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
        });
    }
    Ok(())
}
