use std::thread;

use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode, UnknownReturnValue},
};
use napi_derive::napi;

/// ## 调用后结果
/// \[0\] \[1\] ..
///
/// ts_args_type: 强制指定参数类型
#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(cb: ThreadsafeFunction<u32, UnknownReturnValue>) -> Result<()> {
    for n in 0..100 {
        let cb = cb.clone();
        thread::spawn(move || {
            // 通过 tsfn.call 来调用 JS 函数
            cb.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
        });
    }
    Ok(())
}
