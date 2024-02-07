extern crate objc;

use std::ffi::{c_char, CStr};

use objc::class;
use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};
use urlencoding::decode;

pub fn get_finder() {}

pub fn get_mac_app_path() {
    // unsafe 代码块，表示开发者知道里面的代码不安全
    // 因为要和另一门语言交互，rust必须要确认开发者知道这是不安全的操作
    unsafe {
        // 获取Mac系统的 AppKit 这个模块下的 NSWorkspace 这个对象
        let ns_workspace_class = class!(NSWorkspace);

        // 调用 NSWorkspace 的 sharedWorkspace 方法，拿到 sharedWorkspace
        let shared_workspace: *mut Object = msg_send![ns_workspace_class, sharedWorkspace];

        // 调用 sharedWorkspace 的 frontmostApplication 方法，拿到 frontmostApplication
        // 也就是当前处于前台的 app 对象
        let app: *mut Object = msg_send![shared_workspace, frontmostApplication];

        // 这里可以调用 app 上面的 hide 方法，把这个软件隐藏，相当于系统快捷键 command + h
        // let _result: *mut Object = msg_send![app, hide];

        // 拿到 app 的软件放置的磁盘位置的 NSURL 对象
        let bundle_url: *mut Object = msg_send![app, bundleURL];

        // 把 NSURL 转为 NSString，因为 NSURL 继承于 NSObject 这个几类，所以可以使用 description 这个方法
        let description_string: *mut Object = msg_send![bundle_url, description];

        // 后续操作，是把 object-c 中的 NSString 转为 Rust 的 String 字符串，以便能在 rust 使用
        let description: *mut c_char = msg_send![description_string, UTF8String];
        let c_str: &CStr = CStr::from_ptr(description);
        let app_path = c_str.to_string_lossy().into_owned();

        let parse = decode(&app_path).unwrap();

        dbg!(parse);

        // 打印结果：
        // file:///Applications/Visual Studio Code.app/
    }
}
