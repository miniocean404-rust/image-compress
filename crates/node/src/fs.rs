use std::{fs::File, io::Read, thread::spawn};

use napi::{
    threadsafe_function::{
        ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
    },
    Error, JsFunction, Result,
};
use napi_derive::napi;

fn sync_read_file(path: String) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// napi-线程安全：https://github.com/nodejs/node-addon-api/blob/main/doc/threadsafe_function.md
#[napi(js_name = "readFile", ts_return_type = "void")]
pub fn read_file(path: String, callback: JsFunction) {
    let tsfn: ThreadsafeFunction<Result<String>, ErrorStrategy::CalleeHandled> = callback
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
        tsfn.call(Ok(contents), ThreadsafeFunctionCallMode::Blocking);
    });
}
