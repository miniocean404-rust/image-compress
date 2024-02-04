use backtrace::Backtrace;
use image_compress_core::compress::index::ImageCompression;
/// import the preludes
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::env;
use std::panic::set_hook;

#[napi]
// 定义 export const
pub const DEFAULT_COST: u32 = 12;

#[napi(js_name = "ImageCompression")]
#[derive(Default)]
pub struct ImageCompressionInner {
    inner: ImageCompression,
}

#[napi::module_init]
fn init() {
    if cfg!(debug_assertions) || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("Panic: {:?}\nBacktrace: {:?}", panic_info, backtrace);
        }));
    }
}

#[napi(js_name = "get_image_info")]
#[tracing::instrument(level = "info", skip_all)]
async fn get_image(file: String) -> Result<Buffer> {
    // 如果没用自定义初始化就使用 默认 的初始化
    crate::log::init_default_trace_subscriber();

    let mut info = ImageCompression::new(file, 80).map_err(|e| {
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

// #[napi(ts_return_type = "Promise<{ [index: string]: { code: string, map?: string } }>")]
