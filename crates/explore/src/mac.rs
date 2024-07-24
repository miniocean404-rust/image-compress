use objc::runtime;
#[cfg(target_os = "macos")]
use objc::{msg_send, runtime::Class, sel, sel_impl};
use urlencoding::decode;

use std::{ffi::CStr, process::Command};

#[cfg(target_os = "macos")]
// 必须！：表示引入 Mac 的 AppKit 这个模块，因为要使用这个模块下的对象
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[cfg(target_os = "macos")]
pub fn get_line() {
    let output = Command::new("osascript")
        .arg("get_finder_path.scpt")
        .output()
        .expect("Failed to execute command");

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(target_os = "macos")]
pub fn get_finder() -> anyhow::Result<()> {
    let cls = Class::get("NSWorkspace").unwrap();
    let shared_workspace: *mut objc::runtime::Object = unsafe { msg_send![cls, sharedWorkspace] };

    let running_apps: *mut objc::runtime::Object =
        unsafe { msg_send![shared_workspace, runningApplications] };

    let count: usize = unsafe { msg_send![running_apps, count] };

    for i in 0..count {
        let app: *mut objc::runtime::Object = unsafe { msg_send![running_apps, objectAtIndex:i] };

        let bundle_identifier: *mut objc::runtime::Object =
            unsafe { msg_send![app, bundleIdentifier] };

        if !bundle_identifier.is_null() {
            let bundle_id: &str = unsafe {
                let c_str: *const std::os::raw::c_char = msg_send![bundle_identifier, UTF8String];
                std::ffi::CStr::from_ptr(c_str).to_str()?
            };

            if bundle_id == "com.apple.finder" {
                let is_active: bool = unsafe { msg_send![app, isActive] };

                if is_active {
                    println!("Finder 被激活了");
                } else {
                    println!("Finder正在运行，但是没有激活");
                }

                break;
            }
        }
    }

    Ok(())
}

// https://juejin.cn/post/7208732065696038971
pub fn get_mac_foreground_app_path() {
    unsafe {
        // 获取Mac系统的 AppKit 这个模块下的 NSWorkspace 这个对象
        let cls = Class::get("NSWorkspace").unwrap();

        // 调用 NSWorkspace 的 sharedWorkspace 方法，拿到 sharedWorkspace
        let shared_workspace: *mut runtime::Object = msg_send![cls, sharedWorkspace];

        // 调用 sharedWorkspace 的 frontmostApplication 方法，拿到 frontmostApplication
        // 也就是当前处于前台的 app 对象
        let app: *mut runtime::Object = msg_send![shared_workspace, frontmostApplication];

        // 这里可以调用 app 上面的 hide 方法，把这个软件隐藏，相当于系统快捷键 command + h
        // let _result: *mut Object = msg_send![app, hide];

        // 拿到 app 的软件放置的磁盘位置的 NSURL 对象
        let bundle_url: *mut runtime::Object = msg_send![app, bundleURL];

        // 把 NSURL 转为 NSString，因为 NSURL 继承于 NSObject 这个几类，所以可以使用 description 这个方法
        let description_string: *mut runtime::Object = msg_send![bundle_url, description];

        // 后续操作，是把 object-c 中的 NSString 转为 Rust 的 String 字符串，以便能在 rust 使用
        let description: *mut std::os::raw::c_char = msg_send![description_string, UTF8String];
        let c_str: &CStr = CStr::from_ptr(description);
        let app_path = c_str.to_string_lossy().into_owned();

        let parse = decode(&app_path).unwrap();

        dbg!(parse);

        // 打印结果：
        // file:///Applications/Visual Studio Code.app/
    }
}
