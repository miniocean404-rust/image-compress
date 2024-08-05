use std::{fs::File, io::Read, thread::spawn};

use napi::{
    threadsafe_function::{
        ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    },
    Error, JsFunction, Result, Status,
};
use napi_derive::napi;

use utils::path::deep::get_deep_dirs;

// 掘金文章：https://juejin.cn/post/7322288075850039359?searchId=202402040121440A1FC55F67DF117FA08B
// napi-线程安全：https://github.com/nodejs/node-addon-api/blob/main/doc/threadsafe_function.md
#[napi(js_name = "readFile", ts_return_type = "void")]
pub fn read_file(path: String, callback: JsFunction) {
    let thread_safe_fn: ThreadsafeFunction<Result<String>, ErrorStrategy::CalleeHandled> = callback
        .create_threadsafe_function(
            0,
            |ctx: ThreadSafeCallContext<std::prelude::v1::Result<String, Error>>| match ctx.value {
                Ok(value) => {
                    let js_contents = ctx.env.create_string(&value).unwrap();
                    Ok(vec![js_contents])
                }
                Err(err) => Err(err),
            },
        )
        .unwrap();

    spawn(move || {
        let contents = sync_read_file(path);
        thread_safe_fn.call(Ok(contents), ThreadsafeFunctionCallMode::Blocking);
    });
}

fn sync_read_file(path: String) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[napi(js_name = "getPaths", ts_return_type = "string[]")]
pub fn get_dirs(pattern: String, path: String, max_deep: u32) -> Result<Vec<String>> {
    get_deep_dirs(&pattern, &path, max_deep as usize)
        .map_err(|e| Error::new(Status::GenericFailure, format!("失败的获取路径: {}", e)))
}
