#[cfg(target_os = "macos")]
use crate::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use crate::windows::index::get_explore_info;

mod macos;
mod windows;

pub unsafe fn get_os_dir() -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    let info = unsafe { get_finder_info()? };

    #[cfg(target_os = "windows")]
    let info = unsafe { get_explore_info()? };

    dbg!(info);

    anyhow::Ok(())
}
