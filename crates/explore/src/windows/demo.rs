#![cfg(target_os = "windows")]

// windows api 的A、W、ExA和ExW的区别
// A表示使用ANSI编码作为标准输入与输出流的文本编码
// W表示使用Unicode作为编码
// Ex表示拓展, 标注了Ex的winapi函数会比没有标Ex的函数多一些参数什么的, 可以说拓展了一些功能
// ExA 与 ExW 就是 A,W与Ex的结合了

use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{
    BOOL, COLORREF, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WPARAM,
};
use windows::Win32::Graphics::Gdi::{GetSysColorBrush, COLOR_WINDOW};
// 需要使用 Win32_System_LibraryLoader future
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetSystemMetrics, LoadCursorW,
    PostQuitMessage, RegisterClassW, SetTimer, ShowWindow, TranslateMessage, CS_HREDRAW,
    CS_VREDRAW, IDC_ARROW, MSG, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, WNDCLASSW, WS_POPUP,
};

// demo: https://github.com/microsoft/windows-rs/issues/2427
// demo: https://www.bilibili.com/read/cv17317342/
pub unsafe fn create_window() {
    let instance = HINSTANCE::from(get_module_hwnd());
    // Load the default application icon
    // let h_icon = unsafe { LoadIconW(None, IDI_APPLICATION).unwrap() };
    let class_name = w!("MyWindowClass");

    let wndclass = WNDCLASSW {
        cbClsExtra: 0, // 窗口扩展
        cbWndExtra: 0, // 窗口实例扩展
        // hbrBackground: CreateSolidBrush(create_colorref(0, 0, 0)), // 窗口背景色
        hbrBackground: GetSysColorBrush(COLOR_WINDOW), // 窗口背景色
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(), // 窗口鼠标光标
        hInstance: instance,                           // 实例句柄
        lpfnWndProc: Some(wnd_proc),                   // 定义窗口处理函数
        // lpszClassName: PCWSTR(HSTRING::from("MyWindowClass").as_wide().as_ptr()), // 窗口类名为“窗口”
        // lpszClassName: PCWSTR(HSTRING::from("MyWindowClass\0").as_wide().as_ptr()),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        style: CS_HREDRAW | CS_VREDRAW, //窗口类的风格 | CS_HREDRAW: 当水平长度改变或移动窗口时，重画整个窗口 | CS_VREDRAW: 当垂直长度改变或移动窗口时，重画整个窗口
        // hIcon:h_icon, // 窗口图标
        ..Default::default()
    };

    RegisterClassW(&wndclass);

    let _classname = "MyWindowClass"
        .encode_utf16()
        .collect::<Vec<u16>>()
        .as_ptr();
    let title = "创建的窗口".encode_utf16().collect::<Vec<u16>>().as_ptr();

    let hwnd = CreateWindowExW(
        Default::default(),
        // WS_EX_LAYERED | WS_EX_TOOLWINDOW, // WS_EX_LAYERED：创建一个分层窗口 | WS_EX_TOOLWINDOW：创建工具窗口，即窗口是一个游动的工具条。
        // PCWSTR::from_raw(classname),      // 窗口类名,需要先注册窗口类
        class_name,
        PCWSTR::from_raw(title), // 窗口名
        WS_POPUP,                // WS_POPUP：创建一个弹出式窗口。
        0,                       // 起点 CW_USEDEFAULT
        0,
        300, //大小
        300,
        None, //父窗口句柄
        None,
        instance, // 模块句柄
        // None 或者 std::ptr::null(),
        None,
    );

    let hwnd = hwnd.unwrap();

    // let success = show_window(hwnd.clone());
    // dbg!(success);

    let success = ShowWindow(hwnd, SW_SHOW);
    let is_success: bool = success.into();
    dbg!(is_success);

    // 得到窗口消息、翻译、发送给消息处理函数
    // Run the message loop
    let mut msg = MSG::default();
    loop {
        let result = GetMessageW(&mut msg, hwnd, 0, 0);
        if result == BOOL(0) {
            break;
        } else if result == BOOL(-1) {
            //handle error
        } else {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    // Clean up
    PostQuitMessage(0);
}

// 设置窗口位置
// HWND_TOPMOST将窗口置于所有非顶层窗口之上。即使窗口未被激活窗口也将保持顶级位置。 | SWP_NOSIZE：维持当前尺寸（忽略cx和Cy参数）| SWP_NOMOVE：维持当前位置（忽略X和Y参数）。
// SetWindowPos(
//     hwnd.clone(),
//     HWND_TOPMOST,
//     0,
//     0,
//     0,
//     0,
//     SWP_NOSIZE | SWP_NOMOVE,
// )
// .unwrap();

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
    DefWindowProcW(hwnd, message, w_param, l_param) // 系统默认处理

    // 消息与左键按位与为true，则执行
    // if (w_param.0 as u32 & MK_LBUTTON) != 0 {
    //     let x = l_param.0 as i32;
    //     let y = l_param.0 as i32;
    //     NOW.x = x << 16 >> 16; //得到鼠标x坐标（l_param低位）
    //     NOW.y = y >> 16; //得到鼠标y坐标（l_param高位）
    //     I_COUNT += 1;
    //
    //     if I_COUNT % 4 == 0 {
    //         InvalidateRect(hwnd, std::ptr::null(), true); //发送重绘窗口消息
    //     }
    // }
}

// 获取一个应用程序或动态链接库的模块句柄，None 则返回调用进程本身的句柄。
pub unsafe fn get_module_hwnd() -> HMODULE {
    GetModuleHandleW(None).unwrap()
}

// 获取系统分辨率
pub unsafe fn get_system_resolution() {
    // SM_CXSCREEN,SM_CYSCREEN 以像素为单位计算的屏幕尺寸
    let cx = GetSystemMetrics(SM_CXSCREEN);
    let cy = GetSystemMetrics(SM_CYSCREEN);
    dbg!(cx, cy);
}

pub fn create_colorref(r: u32, g: u32, b: u32) -> COLORREF {
    COLORREF(r | (g << 8) | (b << 16))
}

// 显示窗口 SW_SHOWNORMAL 默认显示
pub unsafe fn show_window(hwnd: HWND) -> bool {
    // SW_SHOWNORMAL
    let is_success = ShowWindow(hwnd, SW_SHOW);
    is_success.into()
}

// 设置定时器
pub unsafe fn set_timeout(hwnd: HWND) {
    // 发送WM_TIMER消息
    SetTimer(hwnd, 1, 1, None);
}

pub unsafe fn get_mouse_position(_hwnd: HWND) {
    // SetCapture(hwnd);
    // GetCursorPos(&mut START).unwrap();
}

pub unsafe fn other() {
    // 重绘窗口，并发送一个 WM_PAINT
    // UpdateWindow(hwnd);

    // 设置窗口透明度渐变
    //LWA_ALPHA时：crKey参数无效，bAlpha参数有效；
    // SetLayeredWindowAttributes(hwnd, 0, I_ALPHA, LWA_ALPHA);
    // if I_ALPHA > 128 {
    //     KillTimer(hwnd, 1);
    // }
    // I_ALPHA += 10;

    // 窗口绘图操作
    // let hdc = BeginPaint(hwnd, &mut ps);//开始绘图
    // let h_brush_rect = CreateSolidBrush(rgb(233, 233, 233));//创建封闭图形填充刷子
    // let h_brush_frame = CreateSolidBrush(rgb(255, 255, 255));
    // SetRect(&mut rect, min(START.x, NOW.x), min(START.y, NOW.y), max(START.x, NOW.x), max(START.y, NOW.y));//设置矩形坐标
    // FillRect(hdc, &rect, h_brush_rect);//指定的画刷填充矩形
    // FrameRect(hdc, &rect, h_brush_frame);//用指定的画刷为指定的矩形画边框
    // SetLayeredWindowAttributes(hwnd, transparent_color, I_ALPHA, LWA_COLORKEY | LWA_ALPHA);//设置窗口透明度
    // DeleteObject(h_brush_frame);//删除画刷
    // DeleteObject(h_brush_rect);
    // EndPaint(hwnd, &ps);//结束绘图

    // 得到指定矩形的位图并放入剪切板
    // let hdc = GetDC(None); //得到当前模块DC
    // let absx = (START.x - NOW.x).abs();
    // let absy = (START.y - NOW.y).abs();
    // let minx = min(START.x, NOW.x);
    // let miny = min(START.y, NOW.y);
    // let h_bitmap = CreateCompatibleBitmap(hdc, absx, absy); //该函数创建与指定的设备环境相关的设备兼容的位图。位图的absx宽absy高。
    // let hmendc = CreateCompatibleDC(hdc); //该函数创建一个与指定设备兼容的内存设备上下文环境
    // let holdbmp = SelectObject(hmendc, h_bitmap); //给DC送位图，得到holdbmp
    //
    // BitBlt(hmendc, 0, 0, absx, absy, hdc, minx, miny, SRCCOPY); //SRCCOPY：将源矩形区域直接拷贝到目标矩形区域。
    //
    // //将位图保存至临时文件夹
    // let mut bitmap_info = BITMAPINFO::default();//初始化BITMAPINFO
    // bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as _;
    // GetDIBits(hmendc, h_bitmap, 0, absy as _, std::ptr::null_mut(), &mut bitmap_info, DIB_RGB_COLORS);//第一次调用得到biSizeImage
    // bitmap_info.bmiHeader.biCompression = 0;
    // let mut bitmap_bits: Vec<u8> = vec![0; bitmap_info.bmiHeader.biSizeImage as _];
    // GetDIBits(hmendc, h_bitmap, 0, absy as _, bitmap_bits.as_mut_ptr() as _, &mut bitmap_info, DIB_RGB_COLORS);//第二次调用写入位图数据
    // create_bmp_file(bitmap_info, bitmap_bits);
    //
    // //将位图保存至剪切板
    // OpenClipboard(hwnd); //打开剪切板
    // EmptyClipboard(); //清空剪切板
    // SetClipboardData(CF_BITMAP.0, HANDLE(h_bitmap.0)).unwrap(); //设置剪切板数据
    // CloseClipboard(); //关闭剪切板
    //
    // ReleaseCapture(); //释放捕获鼠标
    // SelectObject(hmendc, holdbmp); //取回DC位图
    // DeleteDC(hmendc); //删除DC
    // DeleteObject(h_bitmap); //删除位图
    // ReleaseDC(None, hdc); //释放DC
    // DestroyWindow(hwnd);//销毁窗口

    // 窗口关闭消息处理
    // WM_DESTROY => {
    //     PostQuitMessage(0);
    // }
    // _ => ()

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