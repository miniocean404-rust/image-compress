#![cfg(target_os = "windows")]
#![allow(renamed_and_removed_lints)]
#![allow(missing_safety_doc)]

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetAncestor, GetClassNameW, GetForegroundWindow, GetWindowInfo,
    GetWindowModuleFileNameW, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    GA_ROOT, WINDOWINFO, WS_VISIBLE,
};

// 获取前台应用窗口句柄
pub unsafe fn get_foreground_window() -> HWND {
    GetForegroundWindow()
}

// 与 GetForegroundWindow 类似，但是 GetForegroundWindow 返回的是前台窗口的句柄，而 FindWindowW 是通过类名获取资源管理器窗口的根句柄（只要打开了，包括激活或未激活的状态的句柄）
// 程序：
// explorer.exe: 这是Windows资源管理器的主要执行文件，负责提供图形界面用于访问、修改、删除和创建文件等操作。它也负责显示桌面、任务栏、开始菜单等。
// SearchApp.exe: 这是Windows操作系统中用于搜索的应用程序。在Windows 10和更高版本中，它负责处理开始菜单的搜索功能，允许用户搜索文件、应用程序、设置等。
// SearchHost.exe: 这是Windows搜索服务的一部分，负责处理系统搜索操作的后台进程。它在用户进行搜索时运行，帮助提高搜索结果的相关性和速度。
// FESearchHost.exe: 这个进程与Windows的文件资源管理器搜索功能相关。"FE"可能代表"File Explorer"（文件资源管理器）。这个进程可能是负责处理文件资源管理器内部搜索请求的后台服务。
// prevhost.exe: 这是预览处理程序主机的执行文件，用于Windows操作系统中的预览窗格。当用户在文件资源管理器中选择文件时，此程序负责生成文件的预览，如文档的内容或图片的缩略图。
// 类：
// ExploreWClass：这个类名通常与资源管理器的进程相关联，但它不是所有版本的 Windows 中都使用。在某些特定情况下或特定版本的 Windows 中，ExploreWClass 可能用于资源管理器或其他与文件浏览相关的窗口。
// CabinetWClass：这是最常见的资源管理器窗口类名，用于标识标准的文件浏览窗口。CabinetWClass 窗口类是资源管理器中用于显示文件和文件夹的主要窗口类。
#[allow(dead_code)]
pub unsafe fn get_explore_root_window() -> Option<HWND> {
    // 桌面类名
    // Progman
    // WorkerW
    let class_name = "CabinetWClass"
        .encode_utf16()
        // 方法在 Rust 中用于将单个元素追加到迭代器的末尾，这种方法通常用于处理需要以空终止符结尾的字符串，特别是在与 C API 或 Windows API 交互时，这些 API 需要字符串以空终止符结尾。这里的 0 作为 UTF-16 字符串的空终止符，表示字符串的结束
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    // FindWindowW 搜索顶级窗口、FindWindowExW 搜索顶级窗口及子窗口
    if let Ok(cabinet_hwnd) = FindWindowW(PCWSTR(class_name.as_ptr()), None) {
        if !cabinet_hwnd.0.is_null() {
            return Some(cabinet_hwnd);
        }
    }

    let class_name = "ExploreWClass"
        .encode_utf16()
        // 方法在 Rust 中用于将单个元素追加到迭代器的末尾，这种方法通常用于处理需要以空终止符结尾的字符串，特别是在与 C API 或 Windows API 交互时，这些 API 需要字符串以空终止符结尾。这里的 0 作为 UTF-16 字符串的空终止符，表示字符串的结束
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    if let Ok(explore_hwnd) = FindWindowW(PCWSTR(class_name.as_ptr()), None) {
        if !explore_hwnd.0.is_null() {
            return Some(explore_hwnd);
        }
    }

    None
}

// 获取根窗口句柄
#[allow(dead_code)]
pub unsafe fn get_root_window_hwnd(window: HWND) -> HWND {
    GetAncestor(window, GA_ROOT)
}

pub unsafe fn get_window_title_classname(window: HWND) -> String {
    let mut classname = [0u16; 512];
    let len = GetClassNameW(window, &mut classname);

    OsString::from_wide(&classname[..len as usize])
        .to_string_lossy()
        .to_string()
}

// 获取窗口标题
pub unsafe fn get_window_title(window: HWND) -> String {
    // 声明一个 u16 数组，存储 256 个字符
    let mut title = [0u16; 512];
    let len = GetWindowTextW(window, &mut title);

    String::from_utf16_lossy(&title[..len as usize])

    // GetWindowTextW(hwnd, &mut title);
    // let title = OsString::from_wide(&title[..]);
    // println!("子窗口标题: {}", title.to_str().unwrap_or(""));
}

// 获取窗口标题长度
#[allow(dead_code)]
pub unsafe fn get_window_title_len(window: HWND) -> i32 {
    GetWindowTextLengthW(window)
}

// 不知道是做什么的：检索与指定窗口句柄关联的模块的完整路径和文件名。
#[allow(dead_code)]
pub unsafe fn get_window_program_path(window: HWND) -> String {
    let mut path: [u16; 512] = [0; 512];
    let path_len = GetWindowModuleFileNameW(window, &mut path);
    String::from_utf16_lossy(&path[..path_len as usize])
}

// 根据句柄获取窗口的可执行文件路径
pub unsafe fn get_window_exec_path(window: HWND) -> anyhow::Result<String> {
    let mut process_id = 0;

    GetWindowThreadProcessId(window, Some(&mut process_id));
    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    )?;

    // if process_handle.0 == 0 {
    //     return Err(anyhow::anyhow!("打开进程失败"));
    // }

    let mut buffer: [u16; 1024] = [0; 1024];
    let len = GetModuleFileNameExW(process_handle, None, &mut buffer);

    // if len == 0 {
    //     return Err(anyhow::anyhow!("无法获取模块文件名"));
    // }

    let path = String::from_utf16_lossy(&buffer[..len as usize]);

    CloseHandle(process_handle)?;

    anyhow::Ok(path)
}

// 获取窗口位置信息、可见性
#[allow(dead_code)]
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

// HWND 是一种数据类型，表示窗口句柄（Handle to a Window）。
// EnumWindows(Some(enum_windows), LPARAM(0)).unwrap(); // EnumWindows 是一个 Windows API 函数，用于枚举所有顶级窗口。
#[allow(dead_code)]
pub unsafe extern "system" fn enum_windows(window: HWND, _: LPARAM) -> BOOL {
    let info = get_window_info(window).unwrap();
    let title = get_window_title(window);
    let path = get_window_program_path(window);

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

// 获取子窗口句柄
// EnumChildWindows(explore_window, Some(enum_child_proc), LPARAM(0));
#[allow(dead_code)]
pub unsafe extern "system" fn enum_child_proc(window: HWND, _: LPARAM) -> BOOL {
    let classname = get_window_title_classname(window);

    if classname == "ShellTabWindowClass" {
        let title = get_window_title(window);
        // 在这里处理每个子窗口的类名和标题
        println!("子窗口类名: {}", classname);
        println!("子窗口标题: {}", title);
        println!(" ",);
    }

    true.into()
}
