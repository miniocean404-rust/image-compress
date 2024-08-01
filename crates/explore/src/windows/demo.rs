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
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

// demo: https://github.com/microsoft/windows-rs/issues/2427
// demo: https://www.bilibili.com/read/cv17317342/
pub unsafe fn create_window() -> anyhow::Result<()> {
    let instance = HINSTANCE::from(get_module_hwnd());

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

    // let _hwnd_stratify = CreateWindowExW(
    //     WS_EX_LAYERED | WS_EX_TOOLWINDOW,
    //     w!("Stratify"),
    //     w!("Stratify"),
    //     WS_POPUP,
    //     410,
    //     10,
    //     80,
    //     20,
    //     None,
    //     None,
    //     instance,
    //     None,
    // )?;

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

pub fn loword(l: usize) -> usize {
    l & 0xffff
}

pub fn hiword(l: usize) -> usize {
    (l >> 16) & 0xffff
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
                window_draw(hwnd);
            }
        }
        // 定时器消息，需要 set_timeout(hwnd_main) 开启定时器
        WM_TIMER => {
            set_window_transparent(hwnd);
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

    // 消息与按位与为 true，则执行
    // if (w_param.0 as u32 & MK_LBUTTON) != 0 {

    // }
}

// 获取一个应用程序或动态链接库的模块句柄，None 则返回调用进程本身的句柄。
pub unsafe fn get_module_hwnd() -> HMODULE {
    GetModuleHandleW(None).unwrap()
}

/// 获取系统分辨率(像素)
// GetSystemMetrics：检索指定的系统指标或系统配置设置
pub unsafe fn get_system_resolution() -> (i32, i32) {
    // SM_CXSCREEN, SM_CYSCREEN 以像素为单位计算的屏幕尺寸
    let cx = GetSystemMetrics(SM_CXSCREEN);
    let cy = GetSystemMetrics(SM_CYSCREEN);

    (cx, cy)
}

pub fn rgb(r: u32, g: u32, b: u32) -> COLORREF {
    COLORREF(r | (g << 8) | (b << 16))
}

/// 显示窗口 SW_SHOWNORMAL 默认显示 如果窗口以前可见，则返回值为非零值。如果以前隐藏窗口，则返回值为零。
///
/// 网址：https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-showwindow
pub unsafe fn show_window(hwnd: HWND) -> BOOL {
    // SW_SHOWNORMAL
    ShowWindow(hwnd, SW_SHOWNORMAL)
}

// 设置窗口绑定定时器 nidevent：定时器 id,uelapse: 为定时器事件之间的时间间隔，单位为毫秒
pub unsafe fn set_timeout(hwnd: HWND) {
    // 发送 WM_TIMER 消息
    SetTimer(hwnd, 1, 1000, None);
}

// 获取鼠标位置，基于屏幕坐标
pub unsafe fn get_mouse_position(hwnd: HWND) {
    SetCapture(hwnd);
    let point: *mut POINT = &mut POINT { x: 0, y: 0 };
    GetCursorPos(point).unwrap();
}

// 设置分层窗口透明度渐变
pub unsafe fn set_window_transparent(hwnd: HWND) {
    // dwFlags 为 LWA_ALPHA 时：crKey 参数无效，bAlpha 参数有效；
    SetLayeredWindowAttributes(hwnd, COLORREF(0), 70, LWA_ALPHA).unwrap();
    // if I_ALPHA > 128 {
    //     // 销毁定时器
    //     KillTimer(hwnd, 1).expect("TODO: panic message");
    // }
}

// 设置窗口位置
pub unsafe fn set_window_position(hwnd: HWND, x: i32, y: i32, width: i32, height: i32) {
    // hWndInsertAfter：HWND_TOPMOST 将窗口置于所有非顶层窗口之上。即使窗口未被激活窗口也将保持顶级位置。
    // uFlags: SWP_NOSIZE：维持当前尺寸（忽略cx和Cy参数）| SWP_NOMOVE：维持当前位置（忽略X和Y参数）。
    SetWindowPos(hwnd, HWND_TOPMOST, x, y, width, height, SWP_FRAMECHANGED).unwrap();
}

pub unsafe fn send_update_window_message(hwnd: HWND) -> BOOL {
    // 重绘窗口，并发送一个 WM_PAINT
    UpdateWindow(hwnd)
}

// 未实现：窗口绘图操作
pub unsafe fn window_draw(hwnd: HWND) {
    let mut ps = PAINTSTRUCT::default();

    let hdc = BeginPaint(hwnd, &mut ps); //开始绘图
    let h_brush_rect = CreateSolidBrush(rgb(233, 233, 233)); //创建封闭图形填充刷子
    let h_brush_frame = CreateSolidBrush(rgb(255, 255, 255));

    let mut rect = RECT::default();

    let _ = SetRect(&mut rect, 10, 10, 20, 20); //设置矩形坐标
    FillRect(hdc, &rect, h_brush_rect); //指定的画刷填充矩形
    FrameRect(hdc, &rect, h_brush_frame); //用指定的画刷为指定的矩形画边框
    SetLayeredWindowAttributes(hwnd, COLORREF(0), 70, LWA_COLORKEY | LWA_ALPHA).unwrap(); //设置窗口透明度
    let _ = DeleteObject(h_brush_frame); //删除画刷
    let _ = DeleteObject(h_brush_rect);
    let _ = EndPaint(hwnd, &ps); //结束绘图
}

pub unsafe fn other() {
    // 得到指定矩形的位图并放入剪切板
    // let hdc = GetDC(None); //得到当前模块DC
    // let absx = (START.x - NOW.x).abs();
    // let absy = (START.y - NOW.y).abs();
    // let minx = min(START.x, NOW.x);
    // let miny = min(START.y, NOW.y);
    // let h_bitmap = CreateCompatibleBitmap(hdc, absx, absy); //该函数创建与指定的设备环境相关的设备兼容的位图。位图的absx宽absy高。
    // let hmendc = CreateCompatibleDC(hdc); //该函数创建一个与指定设备兼容的内存设备上下文环境
    // let holdbmp = SelectObject(hmendc, h_bitmap); //给DC送位图，得到holdbmp
    // BitBlt(hmendc, 0, 0, absx, absy, hdc, minx, miny, SRCCOPY); //SRCCOPY：将源矩形区域直接拷贝到目标矩形区域。

    // //将位图保存至临时文件夹
    // let mut bitmap_info = BITMAPINFO::default();//初始化BITMAPINFO
    // bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as _;
    // GetDIBits(hmendc, h_bitmap, 0, absy as _, std::ptr::null_mut(), &mut bitmap_info, DIB_RGB_COLORS);//第一次调用得到biSizeImage
    // bitmap_info.bmiHeader.biCompression = 0;
    // let mut bitmap_bits: Vec<u8> = vec![0; bitmap_info.bmiHeader.biSizeImage as _];
    // GetDIBits(hmendc, h_bitmap, 0, absy as _, bitmap_bits.as_mut_ptr() as _, &mut bitmap_info, DIB_RGB_COLORS);//第二次调用写入位图数据
    // create_bmp_file(bitmap_info, bitmap_bits);

    // //将位图保存至剪切板
    // OpenClipboard(hwnd); //打开剪切板
    // EmptyClipboard(); //清空剪切板
    // SetClipboardData(CF_BITMAP.0, HANDLE(h_bitmap.0)).unwrap(); //设置剪切板数据
    // CloseClipboard(); //关闭剪切板
    // ReleaseCapture(); //释放捕获鼠标
    // SelectObject(hmendc, holdbmp); //取回DC位图
    // DeleteDC(hmendc); //删除DC
    // DeleteObject(h_bitmap); //删除位图
    // ReleaseDC(None, hdc); //释放DC
    // DestroyWindow(hwnd);//销毁窗口

    // 位图数据保存为bmp文件
    // 创建文件目录
    // let temp_path = std::env::temp_dir().to_str().unwrap().to_string();
    // if !std::path::Path::new(&(temp_path.to_owned() + "/jietutemp")).exists() {
    //     std::fs::create_dir(temp_path.to_owned() + "/jietutemp").unwrap();
    // }

    // //创建文件
    // let mut f = std::fs::File::create(temp_path + "/jietutemp/jietutemp.bmp").unwrap();
    // f.write(b"BM").unwrap();
    // f.write(&(bitmap_info.bmiHeader.biSizeImage + std::mem::size_of::<BITMAPFILEHEADER>() as u32 + std::mem::size_of::<BITMAPINFOHEADER>() as u32).to_le_bytes()).unwrap();
    // f.write(&0u16.to_le_bytes()).unwrap();
    // f.write(&0u16.to_le_bytes()).unwrap();
    // f.write(&(std::mem::size_of::<BITMAPFILEHEADER>() as u32 + std::mem::size_of::<BITMAPINFOHEADER>() as u32).to_le_bytes()).unwrap();
    // f.write(&40u32.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biWidth.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biHeight.to_le_bytes()).unwrap();
    // f.write(&1u16.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biBitCount.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biCompression.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biSizeImage.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biXPelsPerMeter.to_le_bytes()).unwrap();
    // f.write(&bitmap_info.bmiHeader.biYPelsPerMeter.to_le_bytes()).unwrap();
    // f.write(&0u32.to_le_bytes()).unwrap();
    // f.write(&0u32.to_le_bytes()).unwrap();
    // f.write(&bitmap_bits).unwrap();
}
