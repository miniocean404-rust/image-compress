#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

#[derive(Debug, Default)]
pub struct AppInfo {
    // 句柄
    #[cfg(target_os = "windows")]
    pub hwnd_id: HWND,
    // 窗口标题
    pub title: String,
    // MacOS bundleId
    pub bundle_id: String,
    // 是否激活
    pub is_active: bool,
    // 目录路径
    pub dir: String,
    // 执行程序路径
    pub exec: String,
}
