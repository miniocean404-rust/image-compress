use explore::export::dir::get_os_file_manager_path;

fn main() {
    unsafe {
        get_os_file_manager_path().unwrap();
    }
}
