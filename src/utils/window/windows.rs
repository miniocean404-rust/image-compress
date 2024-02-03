#[cfg(windows)]
use std::ptr::{self};

use windows::Win32::{System::Com::CoCreateInstance, UI::Shell::CSIDL_WINDOWS};

pub fn test() {
    // unsafe { CoCreateInstance(&CSIDL_WINDOWS, ptr::null_mut(), 1) };
    // println!("Current folder path: {:?}", path);
}

// #[cfg(windows)]
// pub fn get_windows_program_path() {
//     unsafe {
//         let hwnd = GetForegroundWindow();
//         let mut pid: DWORD = 0;
//         GetWindowThreadProcessId(hwnd, &mut pid);

//         let process_handle = OpenProcess(winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);

//         if !process_handle.is_null() {
//             let mut buffer: Vec<u16> = vec![0; 4096];
//             let mut size: DWORD = buffer.len() as DWORD;
//             if QueryFullProcessImageNameW(process_handle, 0, buffer.as_mut_ptr(), &mut size) != 0 {
//                 let os_str = OsString::from_wide(&buffer[..(size as usize)]);
//                 let process_path = os_str.to_string_lossy();
//                 println!("应用程序路径: {}", process_path);
//             }
//         }
//     }
// }

// Define the ExplorerFolderInfo struct
// #[repr(C)]
// pub struct ExplorerFolderInfo {
//     hwnd: HWND,
//     pidl: LPITEMIDLIST,
// }

// https://pianshen.com/question/68891958968/
// fn get_current_explorer_folders() -> Result<Vec<ExplorerFolderInfo>, String> {
// let mut psh_windows: *mut IShellWindows = std::ptr::null_mut();

// unsafe {
// let hr = CoCreateInstance(
//     &CLSID_ShellWindows,
//     ptr::null_mut(),
//     1u32,
//     &IShellWindows::uuidof(),
//     &mut psh_windows as *mut *mut IShellWindows as *mut *mut c_void,
// );

//     if hr < 0 {
//         return Err(format!("Could not create instance of IShellWindows. Error code: {}", hr));
//     }
// }

// let mut count: i32 = 0;
// unsafe {
//     (*psh_windows).get_Count(&mut count);
// }

// let mut result: Vec<ExplorerFolderInfo> = Vec::with_capacity(count as usize);

// for i in 0..count {
//     let mut info = ExplorerFolderInfo {
//         hwnd: std::ptr::null_mut(),
//         pidl: std::ptr::null_mut(),
//     };

//     // Implement the logic to retrieve information about currently open explorer windows here.

//     result.push(info);
// }

// Ok(result)
// }
