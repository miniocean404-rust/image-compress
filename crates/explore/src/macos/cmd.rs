#![cfg(target_os = "macos")]

use std::process::Command;

use anyhow::anyhow;

// 输出 finder 路径或桌面路径
pub fn get_finder_path() -> anyhow::Result<String> {
    let output = Command::new("osascript")
        .args(["-e",r#"tell application "Finder" to get the POSIX path of (target of front window as alias)"#])
        .output().map_err(|_|anyhow!("获取 Finder 目录路径命令失败"))?;

    if !output.stdout.is_empty() {
        return anyhow::Ok(String::from_utf8_lossy(&output.stdout).to_string());
    }

    if !output.stderr.is_empty() {
        let desktop = Command::new("osascript")
            .args([
                "-e",
                r#"tell application "Finder" to get the POSIX path of (path to desktop)"#,
            ])
            .output()
            .map_err(|_| anyhow!("获取 Desktop 目录路径命令失败"))?;

        return anyhow::Ok(String::from_utf8_lossy(&desktop.stdout).to_string());
    }

    anyhow::Ok("".to_string())
}
