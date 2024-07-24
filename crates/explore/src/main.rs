#[cfg(target_os = "macos")]
use explore::mac;
use explore::windows::explore::get_explore_path;

#[cfg(target_os = "windows")]

fn main() {
    #[cfg(target_os = "macos")]
    mac::get_finder_path();
    #[cfg(target_os = "windows")]
    let path = get_explore_path();
}
