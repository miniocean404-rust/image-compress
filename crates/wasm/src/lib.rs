use wasm_bindgen::{prelude::*, JsCast}; // 用于加载 Prelude（预导入）模块

// 操作 DOM
// start 标识 init() 在 WASM 加载时自动执行
#[wasm_bindgen]
// 斐波那契函数
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen(start)]
pub fn start() {
    println!("start")
}
