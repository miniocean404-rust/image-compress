/// import the preludes
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
fn test(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => test(n - 1) + test(n - 2),
    }
}
