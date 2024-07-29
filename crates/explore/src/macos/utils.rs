#![cfg(target_os = "macos")]

use std::ffi::{c_char, CStr};

use urlencoding::decode;

use objc::runtime;
use objc::{msg_send, runtime::Class, sel, sel_impl};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_app_bundle_id(app: *mut objc::runtime::Object) -> Option<String> {
    let bundle_identifier: *mut objc::runtime::Object = msg_send![app, bundleIdentifier];

    if !bundle_identifier.is_null() {
        let c_str: *const c_char = msg_send![bundle_identifier, UTF8String];
        let bundle_id = CStr::from_ptr(c_str).to_string_lossy().into_owned();

        return Some(bundle_id);
    }

    None
}

pub unsafe fn get_app_title(app: *mut objc::runtime::Object) -> Option<String> {
    // 获取 mainWindow 实例方法
    let main_window: *mut objc::runtime::Object = msg_send![app, mainWindow];

    dbg!(main_window);

    // 获取 title 实例方法
    let title: *const std::os::raw::c_char = msg_send![main_window, title];

    let title_str = CStr::from_ptr(title).to_string_lossy().into_owned();

    println!("{}", title_str);

    None
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_app_is_focus(app: *mut objc::runtime::Object) -> bool {
    msg_send![app, isActive]
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_foreground_app() -> *mut runtime::Object {
    // 获取Mac系统的 AppKit 这个模块下的 NSWorkspace 这个对象
    let cls = Class::get("NSWorkspace").unwrap();

    // 调用 NSWorkspace 的 sharedWorkspace 方法，拿到 sharedWorkspace
    let shared_workspace: *mut runtime::Object = msg_send![cls, sharedWorkspace];

    // 调用 sharedWorkspace 的 frontmostApplication 方法，拿到 frontmostApplication
    // 也就是当前处于前台的 app 对象
    let app: *mut runtime::Object = msg_send![shared_workspace, frontmostApplication];

    app
}

// https://juejin.cn/post/7208732065696038971
// 获取 app 的路径
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_app_exec_path(app: *mut objc::runtime::Object) -> anyhow::Result<String> {
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

    // 打印结果：
    // file:///Applications/Visual Studio Code.app/
    let parse = decode(&app_path)?.to_string();

    anyhow::Ok(parse)
}
