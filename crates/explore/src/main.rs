use explore::win::get_all_explorer;

fn main() {
    let paths = get_all_explorer().unwrap();
    dbg!(paths);
}
