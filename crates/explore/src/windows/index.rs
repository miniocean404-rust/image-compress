// C# https://stackoverflow.com/questions/27590086/c-sharp-get-the-windows-explore-path-which-has-the-focus
// C# https://stackoverflow.com/questions/8292953/get-current-selection-in-windowsexplorer-from-a-c-sharp-application
#![cfg(target_os = "windows")]
#![allow(renamed_and_removed_lints)]
#![allow(missing_safety_doc)]

use crate::dto::app_info::{AppInfo, Platform};
use crate::windows::explore::{get_path_from_explore_view, get_sub_explore};
use crate::windows::utils::{get_foreground_window, get_window_exec_path, get_window_title};

// 获取资源管理器的路径
pub unsafe fn get_explore_info() -> anyhow::Result<AppInfo> {
    let infos = get_sub_explore()?;
    let foreground_window = get_foreground_window();
    let foreground_title = get_window_title(foreground_window);
    let exec = get_window_exec_path(foreground_window)?;

    let mut app_info = AppInfo {
        hwnd_id: foreground_window.0 as isize,
        title: foreground_title.clone(),
        is_active: true,
        exec,
        platform: Platform::Windows,
        ..Default::default()
    };

    infos.iter().try_for_each(|info| {
        let dir = get_path_from_explore_view(&info.shell_browser)?;
        let title = get_window_title(info.hwnd);

        if foreground_title == title {
            app_info.dir = dir;
        }

        anyhow::Ok(())
    })?;

    Ok(app_info)
}
