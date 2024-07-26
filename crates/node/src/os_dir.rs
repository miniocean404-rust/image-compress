use explore::export::dir::get_os_dir;
use napi::{Error, Status};
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

#[napi(js_name = "getOSDir")]
pub fn get_os_dirs() -> napi::Result<TsAppInfo> {
    let info = unsafe {
        let info = get_os_dir().map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("获取系统窗口信息失败:, {}", err),
            )
        })?;

        TsAppInfo {
            title: info.title,
            bundle_id: info.bundle_id,
            is_active: info.is_active,
            dir: info.dir,
            exec: info.exec,
        }
    };

    Ok(info)
}
