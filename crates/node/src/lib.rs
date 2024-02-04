use image_compress_core::compress::index::ImageCompression;
/// import the preludes
use napi::bindgen_prelude::*;
use napi_derive::napi;

// #[napi(js_name = "Compiler")]
// pub struct impl ImageCompression {}

#[napi::module_init]
fn init() {}

#[napi(js_name = "compress_test")]
fn compress(file: String) -> Vec<u8> {
    let mut info = ImageCompression::new(file, 80).unwrap();
    // AsyncTask::with_optional_signal(info.start_mem_compress(false), None);
    info.mem
}

// #[napi(ts_return_type = "Promise<{ [index: string]: { code: string, map?: string } }>")]
