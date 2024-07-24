use std::env;
use std::panic::set_hook;

use backtrace::Backtrace;
use napi_derive::napi;

#[napi::module_init]
fn init() {
    if cfg!(debug_assertions) || env::var("CUSTOM_DEBUG").unwrap_or_default() == "1" {
        set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("恐慌: {:?}\n回溯: {:?}", panic_info, backtrace);
        }));
    }
}

#[napi]
// 定义 export const
pub const DEFAULT_COST: u32 = 12;
