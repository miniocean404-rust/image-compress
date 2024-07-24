#[cfg(target_os = "macos")]
use explore::mac;
use explore::win;

fn main() {
    #[cfg(target_os = "macos")]
    mac::get_os_dir_path();
    #[cfg(target_os = "windows")]
    win::get_os_dir_path().unwrap();
}
