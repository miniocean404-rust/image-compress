use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug, Default)]
pub enum Platform {
    #[default]
    #[strum(serialize = "unknown")]
    Unknown,

    #[strum(serialize = "windows")]
    Windows,

    #[strum(serialize = "macos")]
    MacOS,
}

#[derive(Debug, Default)]
pub struct AppInfo {
    // 句柄
    pub hwnd_id: isize,
    // MacOS bundleId
    pub bundle_id: String,
    // 窗口标题
    pub title: String,
    // 是否激活
    pub is_active: bool,
    // 目录路径
    pub dir: String,
    // 执行程序路径
    pub exec: String,
    // 当前平台
    pub platform: Platform,
}
