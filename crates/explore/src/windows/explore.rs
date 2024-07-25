// RUST https://stackoverflow.com/questions/73311644/get-path-to-selected-files-in-active-explorer-window

#![cfg(target_os = "windows")]

use anyhow::anyhow;
use windows::core::{ComInterface, IUnknown, IntoParam};
use windows::Win32::Foundation::{HWND, S_FALSE};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED,
    COINIT_DISABLE_OLE1DDE,
};
use windows::Win32::System::Ole::IEnumVARIANT;
use windows::Win32::System::Variant::{VARIANT, VT_DISPATCH};
use windows::Win32::UI::Shell::{
    IPersistIDList, IShellBrowser, IShellItem, IShellWindows, IUnknown_QueryService,
    SHCreateItemFromIDList, SID_STopLevelBrowser, ShellWindows, SIGDN_DESKTOPABSOLUTEPARSING,
};

#[derive(Debug)]
pub struct SubExploreInfo {
    pub shell_browser: IShellBrowser,
    pub hwnd: HWND,
}

pub fn get_sub_explore() -> anyhow::Result<Vec<SubExploreInfo>> {
    // CoInitialize 是一个 COM 初始化函数，用于初始化 COM 运行时，可以使用 CoInitialize 及 CoInitializeEx。
    let shell_windows: IShellWindows = unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE)?;
        CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?
    };

    dump_windows(&shell_windows)
}

fn dump_windows(shell_windows: &IShellWindows) -> anyhow::Result<Vec<SubExploreInfo>> {
    let unknowns = unsafe { shell_windows._NewEnum() }?;
    let enum_variant = unknowns.cast::<IEnumVARIANT>()?;

    let mut infos = vec![];

    loop {
        let mut fetched = 0;
        let mut rgvar: [VARIANT; 1] = [VARIANT::default(); 1];
        let hr = unsafe { enum_variant.Next(&mut rgvar, &mut fetched) };

        // 是否没有更多的子窗口
        if hr == S_FALSE || fetched == 0 {
            break;
        }

        // 不是一个 IDispatch 接口？
        if unsafe { rgvar[0].Anonymous.Anonymous.vt } != VT_DISPATCH {
            continue;
        }

        let unk = unsafe {
            rgvar[0]
                .Anonymous
                .Anonymous
                .Anonymous
                .pdispVal
                .as_ref()
                .ok_or(anyhow!("获取 unk 失败"))?
        };

        infos.push(get_browser_info(unk)?);

        // 将 UTF-16 转换为 UTF-8 以供显示
        // let location = String::from_utf16_lossy(&location);
    }

    Ok(infos)
}

// 获取信息
// 设置参数 hwnd: &mut HWND, 可以通过 &mut Default::default() 获取空窗口句柄指针
fn get_browser_info<P>(unk: P) -> anyhow::Result<SubExploreInfo>
where
    P: IntoParam<IUnknown>,
{
    let shell_browser: IShellBrowser =
        unsafe { IUnknown_QueryService(unk, &SID_STopLevelBrowser) }?;
    let hwnd = unsafe { shell_browser.GetWindow() }?;

    Ok(SubExploreInfo {
        shell_browser,
        hwnd,
    })
}

// 从资源管理器视图获取路径, 通过 let location = String::from_utf16_lossy(&location); 获取路径值
pub fn get_path_from_explore_view(browser: &IShellBrowser) -> windows::core::Result<String> {
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

        Ok(String::from_utf16_lossy(&path))
    }
}
