use backtrace::Backtrace;
use image_compress_core::compress::index::ImageCompression;
use image_compress_core::compress::utils::dir::glob_dir;
// use image_compress_core::compress::utils::dir::glob_dir;
/// import the preludes
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::env;
use std::panic::set_hook;

#[napi::module_init]
fn init() {
    if cfg!(debug_assertions) || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("Panic: {:?}\nBacktrace: {:?}", panic_info, backtrace);
        }));
    }
}

#[napi]
// 定义 export const
pub const DEFAULT_COST: u32 = 12;

#[napi(js_name = "getImageBuffer")]
#[tracing::instrument(level = "info", skip_all)]
async fn get_image_buffer(file: String, quality: i8) -> Result<Buffer> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    let mut info = ImageCompression::new(file, quality).map_err(|e| {
        Error::new(
            Status::GenericFailure,
            format!("失败的创建 ImageCompression: {}", e),
        )
    })?;

    info.start_mem_compress(false)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, format!("compress 失败:, {}", e)))?;

    Ok(info.mem.into())
}

#[napi(js_name = "getPaths", ts_return_type = "string[]")]
fn get_dirs(pattern: String, path: String) -> Result<Vec<String>> {
    glob_dir(&pattern, &path)
        .map_err(|e| Error::new(Status::GenericFailure, format!("失败的获取路径: {}", e)))
}
