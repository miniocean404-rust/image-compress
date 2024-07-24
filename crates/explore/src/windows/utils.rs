#![cfg(target_os = "windows")]

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetAncestor, GetForegroundWindow, GetWindowInfo, GetWindowModuleFileNameW, GetWindowTextW,
    GA_ROOT, WINDOWINFO,
};

// ExploreWClass
// CabinetWClass

// 获取前台应用窗口句柄
pub unsafe fn get_foreground_window() -> HWND {
    GetForegroundWindow()
}

// 获取根窗口句柄
pub unsafe fn get_root_window_hwnd(window: HWND) -> HWND {
    GetAncestor(window, GA_ROOT)
}

// 获取窗口标题
pub unsafe fn get_window_title(window: HWND) -> Option<String> {
    // 声明一个 u16 数组，存储 256 个字符
    let mut title = [0u16; 512];
    let len = GetWindowTextW(window, &mut title);

    if len > 0 {
        return Some(String::from_utf16_lossy(&title[..len as usize]));
    }

    None
}

// 未知：检索与指定窗口句柄关联的模块的完整路径和文件名。
pub unsafe fn get_window_program_path(window: HWND) -> Option<String> {
    let mut path: [u16; 512] = [0; 512];
    let path_len = GetWindowModuleFileNameW(window, &mut path);

    if path_len > 0 {
        return Some(String::from_utf16_lossy(&path[..path_len as usize]));
    }

    None
}

// 获取窗口位置信息、可见性
pub unsafe fn get_window_info(window: HWND) -> anyhow::Result<WINDOWINFO> {
    //     // IsWindowVisible 也可以用来判断窗口是否可见
    //     if info.dwStyle.contains(WS_VISIBLE) {
    //         println!("{:?}", info);
    //     }

    let mut info = WINDOWINFO {
        cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
        ..Default::default()
    };
    GetWindowInfo(window, &mut info)?;

    anyhow::Ok(info)
}
