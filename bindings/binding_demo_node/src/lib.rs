use std::env;
use utils::env::is_debug;
use utils::hook::panic_hook::register_panic_hook;

pub mod async_fn;
pub mod callback;
pub mod fs;
pub mod log;
pub mod variable;

#[napi::module_init]
fn init() {
    if is_debug() || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        register_panic_hook()
    }
}
