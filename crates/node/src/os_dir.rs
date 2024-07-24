use napi_derive::napi;

#[napi(js_name = "getOSDir", ts_return_type = "string")]
pub fn get_os_dirs() -> napi::Result<String> {
    Ok("os_dir".to_string())
}
