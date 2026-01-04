#[cfg(target_os = "macos")]
use explore::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use explore::windows::index::get_explore_info;

fn main() {
    unsafe {
        // get_os_file_manager_path().unwrap();
        #[cfg(target_os = "windows")]
        get_explore_info().unwrap();

        #[cfg(target_os = "macos")]
        get_finder_info().unwrap();
    }
}
