use wasm_bindgen::prelude::*;

// 调用 JS 中的方法
// 1. 项目根目录下创建 js2rust 文件夹 并写入内容
#[wasm_bindgen(module = "/js2rust/point.js")]
extern "C" {
    pub type Point;

    #[wasm_bindgen(constructor)]
    fn new(x: i32, y: i32) -> Point;

    #[wasm_bindgen(method, getter)]
    fn get_x(this: &Point) -> i32;

    #[wasm_bindgen(method, getter)]
    fn get_y(this: &Point) -> i32;

    #[wasm_bindgen(method, setter)] //5
    fn set_x(this: &Point, x: i32) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Point, y: i32) -> i32;

    #[wasm_bindgen(method)]
    fn add(this: &Point, p: Point);
}

// 这个函数 JS 侧可以继续进行调用，最终会返回一个 point 对象实例
#[wasm_bindgen]
pub fn test_point() -> Point {
    let p = Point::new(10, 10);
    let p1 = Point::new(6, 3);
    p.add(p1);
    p
}
