use explore::export::dir::get_os_dir;
use napi_derive::napi;

#[napi(object)]
pub struct TsAppInfo {
    // #[napi(ts_type = "MySpecialString")]
    // pub type_override: String,

    // #[napi(ts_type = "object")]
    // pub type_override_optional: Option<String>,

    // 句柄
    // #[cfg(target_os = "windows")]
    // pub hwnd_id: HWND,

    // 窗口标题
    #[napi(ts_type = "string")]
    pub title: String,

    #[napi(ts_type = "string")]
    // MacOS bundleId
    pub bundle_id: String,

    #[napi(ts_type = "bool")]
    // 是否激活
    pub is_active: bool,

    #[napi(ts_type = "string")]
    // 目录路径
    pub dir: String,

    #[napi(ts_type = "string")]
    // 执行程序路径
    pub exec: String,
}

// #[napi(js_name = "getOSDir", ts_return_type = "string")]
// pub fn get_os_dirs() -> napi::Result<String> {
//     unsafe {
//         let info = get_os_dir().unwrap();
//     };
// }
