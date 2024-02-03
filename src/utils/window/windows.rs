extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{self};
use winapi::shared::minwindef::{DWORD, FALSE, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winbase::QueryFullProcessImageNameW;
use winapi::um::winuser::{GetClassNameW, GetForegroundWindow, GetWindowThreadProcessId, SendMessageW};

pub fn get_windows_program_path() {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut pid: DWORD = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);

        let process_handle = OpenProcess(winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);

        if !process_handle.is_null() {
            let mut buffer: Vec<u16> = vec![0; 4096];
            let mut size: DWORD = buffer.len() as DWORD;
            if QueryFullProcessImageNameW(process_handle, 0, buffer.as_mut_ptr(), &mut size) != 0 {
                let os_str = OsString::from_wide(&buffer[..(size as usize)]);
                let process_path = os_str.to_string_lossy();
                println!("应用程序路径: {}", process_path);
            }
        }
    }
}

pub fn get_windows_dir_path() {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_null() {
        println!("Failed to get foreground window handle");
        return;
    }

    let mut class_name: [u16; 256] = [0; 256];
    let length = unsafe { GetClassNameW(hwnd, class_name.as_mut_ptr(), 256) };
    if length == 0 {
        println!("Failed to retrieve class name of the window");
        return;
    }

    let class_name = String::from_utf16_lossy(&class_name[..length as usize]);

    if class_name.contains("CabinetWClass") {
        match get_folder_path_from_file_explorer(hwnd) {
            Some(path) => println!("Current folder path: {}", path),
            None => println!("Failed to retrieve current folder path"),
        }
    } else {
        println!("Current foreground window is not a File Explorer window");
    }
}

fn get_folder_path_from_file_explorer(hwnd: HWND) -> Option<String> {
    let child_hwnd = unsafe {
        winapi::um::winuser::FindWindowExW(
            hwnd,
            ptr::null_mut(),
            "SHELLDLL_DefView\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            ptr::null(),
        )
    };
    if !child_hwnd.is_null() {
        let listview_hwnd = unsafe {
            winapi::um::winuser::FindWindowExW(
                child_hwnd,
                ptr::null_mut(),
                "SysListView32\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
                ptr::null(),
            )
        };
        if !listview_hwnd.is_null() {
            let mut path_buffer: Vec<u16> = vec![0; 260];
            let length = unsafe { SendMessageW(listview_hwnd, 0x100C, 260, &mut path_buffer as *mut _ as LPARAM) }; // LVM_GETITEMTEXTW
            if length > 0 {
                let os_string = OsString::from_wide(&path_buffer[..length as usize]);
                return Some(os_string.to_string_lossy().into_owned());
            }
        }
    }
    None
}
