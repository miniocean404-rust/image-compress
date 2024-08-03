#![cfg(target_os = "windows")]
#![allow(clippy::missing_safety_doc)]

// windows api 的A、W、ExA和ExW的区别
// A表示使用ANSI编码作为标准输入与输出流的文本编码
// W表示使用Unicode作为编码
// Ex表示拓展, 标注了Ex的winapi函数会比没有标Ex的函数多一些参数什么的, 可以说拓展了一些功能
// ExA 与 ExW 就是 A,W与 Ex 的结合了

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
// 需要使用 Win32_System_LibraryLoader future
use windows::Win32::System::SystemServices::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::utils::{
    get_module_hwnd, get_system_resolution, hiword, loword, rgb, set_layer_window_transparent,
    show_window,
};

// demo: https://github.com/microsoft/windows-rs/issues/2427
// demo: https://www.bilibili.com/read/cv17317342/
pub unsafe fn create_window() -> anyhow::Result<()> {
    let instance = HINSTANCE::from(get_module_hwnd().unwrap());

    // PCWSTR(HSTRING::from("MyWindowClass").as_wide().as_ptr())
    let class_name = w!("MyWindowClass");

    let wndclass = WNDCLASSW {
        cbClsExtra: 0,                                       // 窗口扩展
        cbWndExtra: 0,                                       // 窗口实例扩展
        hbrBackground: CreateSolidBrush(rgb(255, 255, 255)), // 窗口背景色  // GetSysColorBrush(COLOR_WINDOW) 获取系统窗口颜色
        hInstance: instance,                                 // 实例句柄
        lpfnWndProc: Some(wnd_proc),                         // 定义窗口处理函数
        lpszClassName: PCWSTR(class_name.as_ptr()),
        style: CS_HREDRAW | CS_VREDRAW, //窗口类的风格 | CS_HREDRAW: 当水平长度改变或移动窗口时，重画整个窗口 | CS_VREDRAW: 当垂直长度改变或移动窗口时，重画整个窗口
        hIcon: LoadIconW(None, IDI_APPLICATION).unwrap(), // 加载默认图标
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(), // 窗口鼠标光标
        ..Default::default()
    };

    RegisterClassW(&wndclass);

    let (cx, cy) = get_system_resolution();
    let main_width = 640;
    let main_height = 480;

    let hwnd_main = CreateWindowExW(
        WINDOW_EX_STYLE(0), // 这两个条件中 WS_EX_LAYERED：创建一个分层窗口 | WS_EX_TOOLWINDOW：创建工具窗口，即窗口是一个游动的工具条。 需要 SetLayeredWindowAttributes 才会显示
        class_name,         // 窗口类名,需要先注册窗口类
        w!("窗口名称"),     // 窗口名
        WS_OVERLAPPEDWINDOW | WS_SYSMENU, // 文档：https://learn.microsoft.com/zh-cn/windows/win32/winmsg/window-styles
        (cx - main_width) / 2,            // 起点 CW_USEDEFAULT
        (cy - main_height) / 2,
        main_width,  // 窗口宽
        main_height, // 窗口高
        None,        // 父窗口句柄
        None,
        instance, // 模块句柄
        // None 或者 std::ptr::null(),
        None,
    )?;

    // 创建文本框
    let _hwnd_text = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("EDIT"),
        w!("我是文本"),
        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(ES_LEFT as u32) | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
        10,
        10,
        300,
        20,
        hwnd_main,
        None,
        instance,
        None,
    )?;

    // 创建按钮
    let _hwnd_ok = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),
        w!("Ok"),
        WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
        320,
        10,
        80,
        20,
        hwnd_main,
        HMENU(IDOK.0 as _),
        instance,
        None,
    )?;

    let _hwnd_cancel = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),
        w!("Cancel"),
        WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
        410,
        10,
        80,
        20,
        hwnd_main,
        HMENU(IDCANCEL.0 as _),
        instance,
        None,
    )?;

    let _ = show_window(hwnd_main);

    // 得到窗口消息、翻译、发送给消息处理函数
    // Run the message loop
    let mut msg = MSG::default();
    loop {
        let result = GetMessageW(&mut msg, hwnd_main, 0, 0);
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
    let notification_code = hiword(w_param.0);
    let control_handle = l_param;

    match message {
        // 按钮消息
        WM_COMMAND => {
            // 判断按钮 HMENU ID
            if control_id == IDOK.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDOK.0).unwrap().0 as isize
            {
                println!("点击了 OK 按钮!");
            } else if control_id == IDCANCEL.0 as usize
                && notification_code == 0
                && control_handle.0 == GetDlgItem(hwnd, IDCANCEL.0).unwrap().0 as isize
            {
                println!("点击了 Cancel 按钮!");
            }
        }
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
        // 定时器消息，需要 set_timeout(hwnd_main) 开启定时器
        WM_TIMER => {
            let _ = set_layer_window_transparent(hwnd, rgb(0, 0, 0), 90);
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
