// #![cfg(target_arch = "wasm32")]

use dom::demo::init_demo;
// 用于加载 Prelude（预导入）模块
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub mod dom;
pub mod marco_utils;

#[wasm_bindgen(start)]
pub fn init_body() -> Result<(), JsValue> {
    init_demo()?;
    Ok(())
}
