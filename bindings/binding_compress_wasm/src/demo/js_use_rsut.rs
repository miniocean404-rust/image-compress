use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Js 调用
/// ```rust
/// async function main() {
///     const module = await import('../pkg/index');
///     module.hello_world();
/// }
/// ```
#[wasm_bindgen]
pub fn hello_world() {
    console::log_1(&JsValue::from_str("Hello World!"));
}

// 发送数组到 JS
#[wasm_bindgen]
pub fn send_array_to_js() -> Box<[JsValue]> {
    vec![
        JsValue::NULL,
        JsValue::UNDEFINED,
        JsValue::from_str("123"),
        JsValue::TRUE,
        JsValue::FALSE,
    ]
    .into_boxed_slice()
}

#[derive(Serialize, Deserialize)]
pub struct Obj {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<i32>>,
    pub field3: [f32; 4],
    pub field4: bool,
    pub field5: String,
}

// 发送对象到 JS
#[wasm_bindgen]
pub fn send_obj_to_js() -> JsValue {
    let mut map = HashMap::new();
    map.insert(0, String::from("ex"));

    let obj = Obj {
        field1: map,
        field2: vec![vec![1, 2], vec![3, 4]],
        field3: [1., 2., 3., 4.],
        field4: true,
        field5: "哈哈哈".to_string(),
    };

    serde_wasm_bindgen::to_value(&obj).unwrap()
}
