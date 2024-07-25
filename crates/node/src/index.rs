use std::env;
use std::panic::set_hook;

use backtrace::Backtrace;

#[napi::module_init]
fn init() {
    if cfg!(debug_assertions) || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("恐慌: {:?}\n回溯: {:?}", panic_info, backtrace);
        }));
    }
}
