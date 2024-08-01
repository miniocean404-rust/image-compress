#[cfg(target_os = "macos")]
use explore::macos::index::get_finder_info;
#[cfg(target_os = "windows")]
use explore::windows::demo::create_window;

fn main() {
    unsafe {
        // get_os_file_manager_path().unwrap();
        #[cfg(target_os = "windows")]
        let _ = create_window();

        #[cfg(target_os = "macos")]
        get_finder_info().unwrap();
    }
}
