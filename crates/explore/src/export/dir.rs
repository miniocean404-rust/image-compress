use crate::dto::app_info::AppInfo;
#[cfg(target_os = "macos")]
use crate::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use crate::windows::index::get_explore_info;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_os_file_manager_path() -> anyhow::Result<AppInfo> {
    #[cfg(target_os = "macos")]
    let info = unsafe { get_finder_info()? };

    #[cfg(target_os = "windows")]
    let info = unsafe { get_explore_info()? };

    anyhow::Ok(info)
}
