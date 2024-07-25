#[cfg(target_os = "macos")]
use explore::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use explore::windows::index::get_explore_info;

fn main() {
    #[cfg(target_os = "macos")]
    let info = unsafe { get_finder_info().unwrap() };

    #[cfg(target_os = "windows")]
    let info = unsafe { get_explore_info().unwrap() };

    println!("{:?}", info)
}
