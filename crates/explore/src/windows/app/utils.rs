#![cfg(target_os = "windows")]
#![allow(clippy::missing_safety_doc)]

use std::io::Write;
use std::{env, fs};

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Ole::{CF_BITMAP, CLIPBOARD_FORMAT};
use windows::Win32::UI::Input::KeyboardAndMouse::{ReleaseCapture, SetCapture};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::{
    Foundation::{BOOL, COLORREF, HMODULE, HWND},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{SetTimer, ShowWindow, SW_SHOWNORMAL},
};

// use windows::core::w;
// use windows::Win32::Graphics::Gdi;
// use windows::Win32::Storage::FileSystem;
// use windows::Win32::System::IO;

pub fn loword(l: usize) -> usize {
    l & 0xffff
}

pub fn hiword(l: usize) -> usize {
    (l >> 16) & 0xffff
}

// 获取一个应用程序或动态链接库的模块句柄，None 则返回调用进程本身的句柄。
pub unsafe fn get_module_hwnd() -> windows::core::Result<HMODULE> {
    GetModuleHandleW(None)
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
// 事件为 WM_TIMER 消息
pub unsafe fn set_timeout(hwnd: HWND, id: usize, time: u32) {
    // 发送 WM_TIMER 消息
    SetTimer(hwnd, id, time, None);
}

pub unsafe fn clear_timeout(hwnd: HWND, id: usize) -> windows::core::Result<()> {
    KillTimer(hwnd, id)
}

// 获取鼠标位置，基于屏幕坐标
pub unsafe fn get_mouse_position(hwnd: HWND) -> windows::core::Result<*mut POINT> {
    SetCapture(hwnd);
    let point: *mut POINT = &mut POINT { x: 0, y: 0 };
    let _ = GetCursorPos(point);

    Ok(point)
}

/// 获取系统分辨率(像素)
// GetSystemMetrics：检索指定的系统指标或系统配置设置
pub unsafe fn get_system_resolution() -> (i32, i32) {
    // SM_CXSCREEN, SM_CYSCREEN 以像素为单位计算的屏幕尺寸
    let cx = GetSystemMetrics(SM_CXSCREEN);
    let cy = GetSystemMetrics(SM_CYSCREEN);

    (cx, cy)
}

// 设置窗口位置
pub unsafe fn set_window_position(
    hwnd: HWND,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> windows::core::Result<()> {
    // hWndInsertAfter：HWND_TOPMOST 将窗口置于所有非顶层窗口之上。即使窗口未被激活窗口也将保持顶级位置。
    // uFlags: SWP_NOSIZE：维持当前尺寸（忽略cx和Cy参数）| SWP_NOMOVE：维持当前位置（忽略X和Y参数）。
    SetWindowPos(hwnd, HWND_TOPMOST, x, y, width, height, SWP_FRAMECHANGED)
}

pub unsafe fn send_update_window_message(hwnd: HWND) -> BOOL {
    // 重绘窗口，并发送一个 WM_PAINT
    UpdateWindow(hwnd)
}

// 设置分层窗口透明度渐变
pub unsafe fn set_layer_window_transparent(
    hwnd: HWND,
    color: COLORREF,
    alpha: u8,
) -> windows::core::Result<()> {
    let percentage = alpha / 100 * 255;
    // dwFlags 为 LWA_ALPHA 时：crKey 参数无效，bAlpha 参数有效；
    SetLayeredWindowAttributes(hwnd, color, percentage, LWA_ALPHA)
    // if I_ALPHA > 128 {
    //     // 销毁定时器
    //     KillTimer(hwnd, 1).expect("TODO: panic message");
    // }
}

// 窗口绘图操作
pub unsafe fn window_draw_rect(
    hwnd: HWND,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    fill_color: COLORREF,
    border: COLORREF,
) {
    let mut paint = PAINTSTRUCT::default();

    let hdc = BeginPaint(hwnd, &mut paint); //开始绘图
    let fill_brush = CreateSolidBrush(fill_color); // 创建封闭图形填充刷子
    let border_brush = CreateSolidBrush(border); // 创建封闭图形边框刷子

    let mut rect = RECT::default();
    let _ = SetRect(&mut rect, start_x, start_y, end_x, end_y); //设置矩形坐标

    FillRect(hdc, &rect, fill_brush); // 指定的画刷填充矩形
    FrameRect(hdc, &rect, border_brush); //用指定的画刷为指定的矩形画边框

    // 删除画刷
    let _ = DeleteObject(border_brush);
    let _ = DeleteObject(fill_brush);

    // 结束绘图
    let _ = EndPaint(hwnd, &paint);
}

pub unsafe fn set_clipboard(
    hwnd: HWND,
    format: CLIPBOARD_FORMAT,
    data_ptr: *mut core::ffi::c_void,
) -> windows::core::Result<()> {
    OpenClipboard(hwnd)?; // 打开剪切板
    EmptyClipboard()?; // 清空剪切板
    SetClipboardData(format.0.into(), HANDLE(data_ptr)).unwrap(); //设置剪切板数据
    CloseClipboard()?; //关闭剪切板

    Ok(())
}

// 得到指定矩形的位图并放入剪切板
pub unsafe fn get_dc(
    window: HWND,
    start_x: i32,
    start_y: i32,
    bitmap_width: i32,
    bitmap_height: i32,
) -> windows::core::Result<()> {
    // GetDC 函数检索指定窗口的工作区或整个屏幕的设备上下文 (DC) 的句柄。 可以在后续 GDI 函数中使用返回的句柄在 DC 中绘制
    let hdc_hwnd = GetDC(None); // 得到当前模块 DC

    // 该函数创建与指定的设备环境相关的设备兼容的位图。
    let bitmap = CreateCompatibleBitmap(hdc_hwnd, bitmap_width, bitmap_height);
    // 该函数创建一个与指定设备兼容的内存设备上下文环境
    let mem_dc_context = CreateCompatibleDC(hdc_hwnd);
    // 指定设备上下文中选择 bitmap 绘制的对象。
    let origin = SelectObject(mem_dc_context, bitmap); // 给 DC 送位图

    // BitBlt 函数执行与像素矩形相对应的颜色数据的位块传输，从指定的源设备上下文传输到目标设备上下文。
    // SRCCOPY：将源矩形区域直接拷贝到目标矩形区域。
    BitBlt(
        mem_dc_context,
        // x,y 为目标矩形起始点开始绘制的坐标
        0,
        0,
        // 绘制矩形的宽高
        bitmap_width,
        bitmap_height,
        // 源矩形上下文句柄
        hdc_hwnd,
        // 从源矩形图像的哪里开始剪裁 目标矩形的宽高
        start_x,
        start_y,
        SRCCOPY,
    )?;

    // 将位图保存至临时文件夹
    let mut bitmap_info = BITMAPINFO::default();
    bitmap_info.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as _;

    // 另一种设置 BITMAPINFO 方式：https://github.com/microsoft/windows-rs/issues/1386
    // 官方设置 BITMAPINFO 方式：https://learn.microsoft.com/zh-cn/windows/win32/gdi/capturing-an-image
    // 第一次调用得到 biSizeImage
    GetDIBits(
        mem_dc_context,
        bitmap,
        // 要检索的第一个扫描行
        0,
        // 要检索的扫描行数。
        bitmap_height as _,
        None,
        &mut bitmap_info,
        DIB_RGB_COLORS,
    );

    bitmap_info.bmiHeader.biCompression = BI_RGB.0;
    let mut bitmap_bits: Vec<u8> = vec![0; bitmap_info.bmiHeader.biSizeImage as _];

    // 第二次调用写入位图数据
    let di_bits = GetDIBits(
        mem_dc_context,
        bitmap,
        0,
        bitmap_height as _,
        Some(bitmap_bits.as_mut_ptr() as _),
        &mut bitmap_info,
        DIB_RGB_COLORS,
    );

    let status = GetLastError();
    println!("get di bits {}{}", status.0, di_bits);

    create_bmp_file(bitmap_info, bitmap_bits);

    // Win32_System_DataExchange future
    // 将位图保存至剪切板
    set_clipboard(window, CF_BITMAP, bitmap.0).unwrap();

    SelectObject(mem_dc_context, origin); // 取回 DC 位图
    let _ = DeleteDC(mem_dc_context); // 删除 DC
    let _ = DeleteObject(bitmap); // 删除位图
    ReleaseDC(None, hdc_hwnd); // 释放 DC
    DestroyWindow(window)?; // 销毁窗口
    ReleaseCapture()?; // 释放捕获鼠标
    Ok(())
}

pub unsafe fn create_bmp_file(bitmap_info: BITMAPINFO, bitmap_bits: Vec<u8>) {
    // 方式 1
    // 位图数据保存为 bmp 文件
    // 创建文件目录
    let cwd = env::current_dir().unwrap();
    let target_buf = cwd.join("tmp");

    if !target_buf.exists() {
        fs::create_dir(target_buf.to_str().unwrap()).unwrap()
    }

    // 创建文件
    let mut f =
        fs::File::create(target_buf.to_string_lossy().to_string() + "/jietutemp.bmp").unwrap();
    f.write_all(b"BM").unwrap();
    f.write_all(
        &(bitmap_info.bmiHeader.biSizeImage
            + size_of::<BITMAPFILEHEADER>() as u32
            + size_of::<BITMAPINFOHEADER>() as u32)
            .to_le_bytes(),
    )
    .unwrap();
    f.write_all(&0u16.to_le_bytes()).unwrap();
    f.write_all(&0u16.to_le_bytes()).unwrap();
    f.write_all(
        &(size_of::<BITMAPFILEHEADER>() as u32 + size_of::<BITMAPINFOHEADER>() as u32)
            .to_le_bytes(),
    )
    .unwrap();
    f.write_all(&40u32.to_le_bytes()).unwrap();
    f.write_all(&bitmap_info.bmiHeader.biWidth.to_le_bytes())
        .unwrap();
    f.write_all(&bitmap_info.bmiHeader.biHeight.to_le_bytes())
        .unwrap();
    f.write_all(&1u16.to_le_bytes()).unwrap();
    f.write_all(&bitmap_info.bmiHeader.biBitCount.to_le_bytes())
        .unwrap();
    f.write_all(&bitmap_info.bmiHeader.biCompression.to_le_bytes())
        .unwrap();
    f.write_all(&bitmap_info.bmiHeader.biSizeImage.to_le_bytes())
        .unwrap();
    f.write_all(&bitmap_info.bmiHeader.biXPelsPerMeter.to_le_bytes())
        .unwrap();
    f.write_all(&bitmap_info.bmiHeader.biYPelsPerMeter.to_le_bytes())
        .unwrap();
    f.write_all(&0u32.to_le_bytes()).unwrap();
    f.write_all(&0u32.to_le_bytes()).unwrap();
    f.write_all(&bitmap_bits).unwrap();

    // 方式 2
    // let mut bmf_header = BITMAPFILEHEADER::default();
    //
    // let bmp_file_handle = FileSystem::CreateFileW(
    //     w!("TestFile.bmp"),
    //     FileSystem::FILE_GENERIC_WRITE.0,
    //     FileSystem::FILE_SHARE_NONE,
    //     None,
    //     FileSystem::CREATE_ALWAYS,
    //     FileSystem::FILE_ATTRIBUTE_NORMAL,
    //     HANDLE::default(),
    // );
    // let status = GetLastError();
    // println!("create file{}", status.0);
    // // 偏移到实际位图位开始的位置。
    // bmf_header.bfOffBits =
    //     size_of::<BITMAPFILEHEADER>() as u32 + size_of::<BITMAPINFOHEADER>() as u32;
    //
    // // 文件大小
    // bmf_header.bfSize = bitmap_info.bmiHeader.biSizeImage
    //     + size_of::<BITMAPFILEHEADER>() as u32
    //     + size_of::<BITMAPINFOHEADER>() as u32;
    // bmf_header.bfReserved1 = 0;
    // bmf_header.bfReserved2 = 0;
    // //bfType must always be BM for Bitmaps
    // bmf_header.bfType = 0x4D42; // BM
    //
    // let mut dw_bytes_written = 0u32;
    //
    // let bmf_header_ptr = ptr::addr_of_mut!(bmf_header);
    // let bmp_info_bmi_header_ptr = ptr::addr_of_mut!(bitmap_info.bmiHeader);
    //
    // FileSystem::WriteFile(
    //     bmp_file_handle,
    //     Some(size_of::<BITMAPFILEHEADER>() as u32),
    //     Some(&mut dw_bytes_written),
    //     Some(0 as *mut IO::OVERLAPPED),
    // );
    // let status = GetLastError();
    // println!("writefile{}", status.0);
    //
    // println!("{}", dw_bytes_written);
    // FileSystem::WriteFile(
    //     bmp_file_handle,
    //     bmp_info_bmi_header_ptr as *mut ffi::c_void,
    //     mem::size_of::<Gdi::BITMAPINFOHEADER>() as u32,
    //     &mut dw_bytes_written,
    //     0 as *mut IO::OVERLAPPED,
    // );
    // let status = GetLastError();
    // println!("writefile{}", status.0);
    // println!("{}", dw_bytes_written);
    //
    // FileSystem::WriteFile(
    //     bmp_file_handle,
    //     bmp_pixels.as_mut_ptr() as *mut ffi::c_void,
    //     bmp_size as u32,
    //     &mut dw_bytes_written,
    //     0 as *mut IO::OVERLAPPED,
    // );
    //
    // let status = GetLastError();
    // println!("writefile{}", status.0);
    // println!("{}", dw_bytes_written);
    //
    // //Close the handle for the file that was created
    // CloseHandle(bmp_file_handle);
    // let status = GetLastError();
    // println!("closehandle {}", status.0);
}
