use explore::export::dir::get_os_dir;

fn main() {
    unsafe {
        get_os_dir().unwrap();
    }
}
