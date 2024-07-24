// C# https://stackoverflow.com/questions/27590086/c-sharp-get-the-windows-explore-path-which-has-the-focus
// C# https://stackoverflow.com/questions/8292953/get-current-selection-in-windowsexplorer-from-a-c-sharp-application
#![cfg(target_os = "windows")]

use anyhow::anyhow;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::WS_VISIBLE;

use crate::windows::utils::{
    get_foreground_window, get_window_info, get_window_program_path, get_window_title,
};

pub unsafe fn test() -> anyhow::Result<()> {
    let foreground_window = get_foreground_window();
    let _info = get_window_info(foreground_window)?;
    let title = get_window_title(foreground_window).ok_or(anyhow!("获取窗口标题失败"))?;
    let path = get_window_program_path(foreground_window)
        .or(Some(String::new()))
        .unwrap();

    if !title.is_empty() {
        println!("窗口名称：{:?} ", title);
        println!("窗口程序路径：{:?} ", path);
        // println!(
        //     "位置：({}, {}) {:?}",
        //     info.rcWindow.left, info.rcWindow.top, info
        // );
    }

    anyhow::Ok(())
}

// HWND 是一种数据类型，表示窗口句柄（Handle to a Window）。
// EnumWindows(Some(enum_windows), LPARAM(0)).unwrap(); // EnumWindows 是一个 Windows API 函数，用于枚举所有顶级窗口。
pub unsafe extern "system" fn enum_windows(window: HWND, _: LPARAM) -> BOOL {
    let info = get_window_info(window).unwrap();
    let title = get_window_title(window).or(Some(String::new())).unwrap();
    let path = get_window_program_path(window)
        .or(Some(String::new()))
        .unwrap();

    if !title.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
        println!("窗口名称：{:?} ", title);
        println!("窗口程序路径：{:?} ", path);
        println!(" ")
        // println!(
        //     "位置：({}, {}) {:?}",
        //     info.rcWindow.left, info.rcWindow.top, info
        // );
    }
    true.into()
}
