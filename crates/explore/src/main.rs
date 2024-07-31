#[cfg(target_os = "windows")]
use explore::windows::demo::create_window;

fn main() {
    unsafe {
        // get_os_file_manager_path().unwrap();
        #[cfg(target_os = "windows")]
        create_window();
    }
}
