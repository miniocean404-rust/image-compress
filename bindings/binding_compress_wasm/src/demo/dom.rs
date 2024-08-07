use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, JsValue};
// 用于加载 Prelude（预导入）模块
use wasm_bindgen::prelude::*;

use crate::console_log;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str); // 将 js 命名空间中的 console.log 方法定义在 Rust 中
}

// 操作 DOM
// start 标识 init() 在 WASM 加载时自动执行

pub fn init_demo() -> Result<(), JsValue> {
    // 使用 web_sys 的 explore 全局对象
    let window = web_sys::window().expect("不存在全局 explore 对象");
    let document = window.document().expect("需要在 explore 上存在 document");
    let body = document.body().expect("document 中需要存在一个 body");

    // 生成 dom 元素
    let input = document
        .create_element("input")?
        .dyn_into::<web_sys::HtmlInputElement>()?;

    let btn = document.create_element("button")?;
    btn.set_text_content(Some("点击计算斐波那契数"));

    let out = document.create_element("h3")?;

    // 操作
    let input = Rc::new(input); // 为了不违背“一个变量只能有一个所有者”的规则，需要使用 Rc 包裹 input 元素，方便在闭包中拿到并使用它的值
    let out = Rc::new(RefCell::new(out)); // 因为需要改变 out 元素的 textContent，需要使用 RefCell 包裹方便去在闭包中把它当做可变变量来改变它
    {
        let out = out.clone(); // 复制一份智能指针
        let input = input.clone();

        // 使用到 wasm_bindgen::closure::Closure，它的作用是打通 Rust 中的闭包和 JS 中的闭包
        let closure = Closure::<dyn Fn()>::new(move || {
            let val = input.value();
            let num = val.parse::<u32>().unwrap();
            let res = num;
            out.borrow_mut()
                .set_text_content(Some(res.to_string().as_str())); // 在这里使用 borrow_mut 把 out 当做可变变量获取出来，并设置 textContent
        });

        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?; // 挂载事件监听器，将闭包函数先转换为 JS 值，再跳过类型判断，再作为回调函数传给 btn
        closure.forget(); // 释放 Rust 对这片堆内存的管理，交给 JS 的 GC 去回收
    }

    body.append_child(&input)?;
    body.append_child(&btn)?;
    body.append_child(&out.borrow())?; // 挂载 DOM 元素节点
    console_log!("初始化成功");
    Ok(())
}
