#![cfg(target_os = "windows")]
#![allow(clippy::missing_safety_doc)]

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
// 需要使用 Win32_System_LibraryLoader future
use windows::Win32::System::SystemServices::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::utils::{
    get_dc, get_module_hwnd, get_system_resolution, hiword, loword, rgb,
    set_layer_window_transparent, show_window, window_draw_rect,
};

pub unsafe fn create_stratify_window() -> anyhow::Result<()> {
    let instance = HINSTANCE::from(get_module_hwnd()?);

    let class_name = w!("Stratify");

    let wndclass = WNDCLASSW {
        cbClsExtra: 0,                                       // 窗口扩展
        cbWndExtra: 0,                                       // 窗口实例扩展
        hbrBackground: CreateSolidBrush(rgb(255, 255, 255)), // 窗口背景色  // GetSysColorBrush(COLOR_WINDOW) 获取系统窗口颜色
        hInstance: instance,                                 // 实例句柄
        lpfnWndProc: Some(wnd_proc),                         // 定义窗口处理函数
        lpszClassName: PCWSTR(class_name.as_ptr()),
        style: CS_HREDRAW | CS_VREDRAW, //窗口类的风格 | CS_HREDRAW: 当水平长度改变或移动窗口时，重画整个窗口 | CS_VREDRAW: 当垂直长度改变或移动窗口时，重画整个窗口
        hIcon: LoadIconW(None, IDI_APPLICATION)?, // 加载默认图标
        hCursor: LoadCursorW(None, IDC_ARROW)?, // 窗口鼠标光标
        ..Default::default()
    };

    RegisterClassW(&wndclass);

    let (cx, cy) = get_system_resolution();
    let main_width = 400;
    let main_height = 400;

    let hwnd_stratify = CreateWindowExW(
        WS_EX_LAYERED | WS_EX_TOOLWINDOW,
        w!("Stratify"),
        w!("分层界面"),
        WS_POPUP,
        (cx - main_width) / 2,
        (cy - main_height) / 2,
        main_width,
        main_height,
        None,
        None,
        instance,
        None,
    )?;

    let _ = show_window(hwnd_stratify);

    set_layer_window_transparent(hwnd_stratify, rgb(0, 0, 0), 100)?;
    window_draw_rect(
        hwnd_stratify,
        0,
        0,
        main_width / 2,
        main_height / 2,
        rgb(167, 183, 97),
        rgb(0, 0, 0),
    );

    get_dc(hwnd_stratify, 0, 0, 1000, 1000)?;

    // 得到窗口消息、翻译、发送给消息处理函数
    // Run the message loop
    let mut msg = MSG::default();
    loop {
        let result = GetMessageW(&mut msg, hwnd_stratify, 0, 0);
        if result == BOOL(0) {
            break;
        } else if result == BOOL(-1) {
            // handle error
            PostQuitMessage(0);
            break;
        } else {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

// 消息处理函数
// WM_CREATE窗口创建
// WM_TIMER定时器
// WM_LBUTTONDOWN鼠标左键按下
// WM_MOUSEMOVE鼠标移动
// WM_LBUTTONUP鼠标左键松开
// WM_PAINT绘图
// WM_DESTROY销毁
unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let control_id = loword(w_param.0);
    let _notification_code = hiword(w_param.0);
    let control_handle = l_param;

    match message {
        // 鼠标左键按下
        WM_LBUTTONDOWN => {
            if MODIFIERKEYS_FLAGS(control_id as u32) == MK_LBUTTON {
                // 获取相对于窗口的坐标
                let _x = loword(control_handle.0 as usize); // 得到鼠标x坐标（l_param 低位）
                let _y = hiword(control_handle.0 as usize); //得到鼠标y坐标（l_param 高位）

                // 发送重绘窗口消息
                let _is_success = InvalidateRect(hwnd, None, TRUE);

                // get_mouse_position(hwnd);
                // set_window_position(hwnd, 300, 300, 100, 100);
            }
        }
        // 窗口销毁
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcW(hwnd, message, w_param, l_param); // 系统默认处理
        }
    };

    LRESULT(0)
}
