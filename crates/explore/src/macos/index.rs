// 整个文件是 macos 才会编译
#![cfg(target_os = "macos")]

use objc::{msg_send, runtime::Class, sel, sel_impl};

use crate::dto::app_info::{AppInfo, Platform};

use super::{
    cmd::get_finder_path,
    utils::{get_app_bundle_id, get_app_exec_path, get_app_is_focus, get_foreground_app},
};

// 必须！：表示引入 Mac 的 AppKit 这个模块，因为要使用这个模块下的对象
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[allow(clippy::missing_safety_doc)]
// #[inline] 当前函数展开（复制代码）到调用位置
#[inline]
pub unsafe fn get_finder_info() -> anyhow::Result<AppInfo> {
    let mut info = get_foreground_app_info()?;

    if info.bundle_id == "com.apple.finder" && info.is_active {
        let path = get_finder_path()?;
        info.dir = path;
        info.platform = Platform::MacOS;
        return anyhow::Ok(info);
    }

    anyhow::Ok(info)
}

// 获取 macos 正在运行的 app 被激活的窗口及 bundleId
#[allow(clippy::missing_safety_doc)]
#[allow(dead_code)]
pub unsafe fn get_running_apps_info() -> anyhow::Result<Vec<AppInfo>> {
    let mut infos = vec![];

    let cls = Class::get("NSWorkspace").unwrap();
    let shared_workspace: *mut objc::runtime::Object = msg_send![cls, sharedWorkspace];

    let running_apps: *mut objc::runtime::Object = msg_send![shared_workspace, runningApplications];
    let count: usize = msg_send![running_apps, count];

    for i in 0..count {
        let app: *mut objc::runtime::Object = msg_send![running_apps, objectAtIndex:i];

        let bundle_id = get_app_bundle_id(app);
        let is_active = get_app_is_focus(app);
        let exec = get_app_exec_path(app)?;

        let info = match bundle_id {
            Some(bundle_id) => AppInfo {
                bundle_id,
                is_active,
                exec,
                ..Default::default()
            },
            None => AppInfo {
                bundle_id: "".to_string(),
                is_active,
                exec,
                ..Default::default()
            },
        };

        // if bundle_id == "com.apple.finder" {
        //     if is_active {
        //         println!("Finder 被激活了");
        //     } else {
        //         println!("Finder正在运行，但是没有激活");
        //     }

        //     break;
        // }

        infos.push(info)
    }

    Ok(infos)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_foreground_app_info() -> anyhow::Result<AppInfo> {
    let app = get_foreground_app();
    let bundle_id = get_app_bundle_id(app);
    let is_active = get_app_is_focus(app);
    let exec = get_app_exec_path(app)?;
    // get_app_title(app);

    let info = match bundle_id {
        Some(bundle_id) => AppInfo {
            bundle_id,
            is_active,
            exec,
            ..Default::default()
        },
        None => AppInfo {
            bundle_id: "".to_string(),
            is_active,
            exec,
            ..Default::default()
        },
    };

    anyhow::Ok(info)
}
