#[cfg(target_os = "macos")]
use explore::macos::mac::get_finder_info;
#[cfg(target_os = "windows")]
use explore::windows::explore::get_explore_path;

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        let info = get_finder_info().unwrap();
        println!("{:?}", info)
    }

    #[cfg(target_os = "windows")]
    let path = get_explore_path();
}
