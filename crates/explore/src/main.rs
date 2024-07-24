#[cfg(target_os = "macos")]
use explore::mac;
use explore::win;

fn main() {
    #[cfg(target_os = "macos")]
    mac::get_finder_path();
    #[cfg(target_os = "windows")]
    win::get_explore_path().unwrap();
}
