use napi::{Error, Status};
use napi_derive::napi;

use explore::dto::app_info::Platform as SDKPlatform;
use explore::export::dir::get_os_file_manager_path;

#[napi(string_enum)]
#[derive(Debug, Default)]
#[allow(non_camel_case_types)]
pub enum Platform {
    #[default]
    unknown,
    windows,
    macos,
}

// 将 SDKPlatform 转换为 Platform
impl From<SDKPlatform> for Platform {
    fn from(platform: SDKPlatform) -> Self {
        match platform {
            SDKPlatform::Unknown => Platform::unknown,
            SDKPlatform::Windows => Platform::windows,
            SDKPlatform::MacOS => Platform::macos,
        }
    }
}

#[napi(object)]
pub struct AppInfo {
    // #[napi(ts_type = "MySpecialString")]
    // pub type_override: String,

    // #[napi(ts_type = "object")]
    // pub type_override_optional: Option<String>,

    // 句柄
    pub hwnd_id: u32,

    // 窗口标题
    #[napi(ts_type = "string")]
    pub title: String,

    #[napi(ts_type = "string")]
    // MacOS bundleId
    pub bundle_id: String,

    #[napi(ts_type = "boolean")]
    // 是否激活
    pub is_active: bool,

    #[napi(ts_type = "string")]
    // 目录路径
    pub dir: String,

    #[napi(ts_type = "string")]
    // 执行程序路径
    pub exec: String,

    pub platform: Platform,
}

#[napi(js_name = "getOsFileManagerPath")]
pub fn get_os_file_manager_path_node() -> napi::Result<AppInfo> {
    let info = unsafe {
        let info = get_os_file_manager_path().map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("获取系统窗口信息失败:, {}", err),
            )
        })?;

        AppInfo {
            hwnd_id: info.hwnd_id as u32,
            title: info.title,
            bundle_id: info.bundle_id,
            is_active: info.is_active,
            dir: info.dir,
            exec: info.exec,
            platform: Platform::from(info.platform),
        }
    };

    Ok(info)
}
