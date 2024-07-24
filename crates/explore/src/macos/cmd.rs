#![cfg(target_os = "macos")]

use std::process::Command;

pub fn get_finder_path() {
    let output = Command::new("osascript")
        .args(["-e",r#"tell application "Finder" to get the POSIX path of (target of front window as alias)"#])
        .output()
        .expect("获取 Finder 目录路径命令失败");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
