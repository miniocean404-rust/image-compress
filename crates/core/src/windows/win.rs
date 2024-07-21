use windows::core::{ComInterface, IUnknown, IntoParam, Result};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, S_FALSE};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};
use windows::Win32::System::Ole::IEnumVARIANT;
use windows::Win32::System::Variant::{VARIANT, VT_DISPATCH};
use windows::Win32::UI::Shell::{
    IPersistIDList, IShellBrowser, IShellItem, IShellWindows, IUnknown_QueryService, SHCreateItemFromIDList, SID_STopLevelBrowser, ShellWindows,
    SIGDN_DESKTOPABSOLUTEPARSING,
};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowInfo, GetWindowModuleFileNameW, GetWindowTextW, WINDOWINFO, WS_VISIBLE};

// 获取应用程序的名称及位置
// HWND 是一种数据类型，表示窗口句柄（Handle to a Window）。
// EnumWindows(Some(enum_windows), LPARAM(0)).unwrap();
pub unsafe extern "system" fn enum_windows(window: HWND, _: LPARAM) -> BOOL {
    // 声明一个 u16 数组，存储 256 个字符
    let mut title = [0u16; 512];
    let mut path: [u16; 512] = [0; 512];

    let len = GetWindowTextW(window, &mut title);
    let cmd_len = GetWindowModuleFileNameW(window, &mut path);

    if len > 0 {
        let path = String::from_utf16_lossy(&path[..cmd_len as usize]);
        let title = String::from_utf16_lossy(&title[..len as usize]);
        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(window, &mut info).unwrap();
        if !title.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
            println!("窗口名称：{:?} ", title);
            println!("窗口程序路径：{:?} ", path);
            println!("位置：({}, {}) {:?}", info.rcWindow.left, info.rcWindow.top, info);
        }
    }

    // 获取前台应用窗口句柄
    // let foreground_hwnd = unsafe { GetForegroundWindow() };
    // // 根据窗口句柄获取窗口信息
    // let mut info = WINDOWINFO { ..Default::default() };
    // unsafe {
    //     GetWindowInfo(foreground_hwnd.to_owned(), &mut info)?;
    //
    //     // IsWindowVisible 也可以用来判断窗口是否可见
    //     if info.dwStyle.contains(WS_VISIBLE) {
    //         println!("{:?}", info);
    //     }
    // }

    true.into()
}

// 获取资源管理器的路径
pub fn get_all_explorer() -> anyhow::Result<Vec<String>> {
    // CoInitialize 是一个 COM 初始化函数，用于初始化 COM 运行时，可以使用 CoInitialize 及 CoInitializeEx。
    let shell_windows: IShellWindows = unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)?;
        CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?
    };
    dump_windows(&shell_windows)
}

fn dump_windows(shell_windows: &IShellWindows) -> anyhow::Result<Vec<String>> {
    let unknowns = unsafe { shell_windows._NewEnum() }?;
    let enum_variant = unknowns.cast::<IEnumVARIANT>()?;
    let mut dirs = vec![];

    loop {
        let mut fetched = 0;
        let mut rgvar: [VARIANT; 1] = [VARIANT::default(); 1];
        let hr = unsafe { enum_variant.Next(&mut rgvar, &mut fetched) };

        // 没有更多的 windows 了吗？
        if hr == S_FALSE || fetched == 0 {
            break;
        }
        // 不是 IDispatch 接口？
        if unsafe { rgvar[0].Anonymous.Anonymous.vt } != VT_DISPATCH {
            continue;
        }

        let location = get_browser_info(
            unsafe { rgvar[0].Anonymous.Anonymous.Anonymous.pdispVal.as_ref().unwrap() },
            // 获取空窗口句柄指针
            &mut Default::default(),
        )?;

        // 将 UTF-16 转换为 UTF-8 以供显示
        let location = String::from_utf16_lossy(&location);

        dirs.push(location.clone());
    }

    Ok(dirs)
}

// 获取信息
fn get_browser_info<P>(unk: P, hwnd: &mut HWND) -> Result<Vec<u16>>
where
    P: IntoParam<IUnknown>,
{
    let shell_browser: IShellBrowser = unsafe { IUnknown_QueryService(unk, &SID_STopLevelBrowser) }?;
    *hwnd = unsafe { shell_browser.GetWindow() }?;

    get_location_from_view(&shell_browser)
}

fn get_location_from_view(browser: &IShellBrowser) -> Result<Vec<u16>> {
    let shell_view = unsafe { browser.QueryActiveShellView() }?;
    let persist_id_list: IPersistIDList = shell_view.cast()?;
    let id_list = unsafe { persist_id_list.GetIDList() }?;

    unsafe {
        let item = SHCreateItemFromIDList::<IShellItem>(id_list)?;
        let mut ptr = item.GetDisplayName(SIGDN_DESKTOPABSOLUTEPARSING)?;

        // 将 UTF-16 字符串复制到 'Vec<u16>'（包括 NULL 终止符）
        let mut path = Vec::new();

        loop {
            let ch = *ptr.0;
            if ch == 0 {
                break;
            }
            path.push(ch);
            ptr.0 = ptr.0.add(1);
        }
        // Cleanup
        CoTaskMemFree(Some(id_list as _));
        Ok(path)
    }
}
