use explore::get_os_dir;

fn main() {
    unsafe { println!("{:?}", get_os_dir().unwrap()) }
}
