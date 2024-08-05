use std::env;

use utils::env::is_debug;
use utils::hook::panic_hook::register_panic_hook;

#[napi::module_init]
fn init() {
    init_panic_hook();
}

fn init_panic_hook() {
    if is_debug() || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        register_panic_hook()
    }
}
