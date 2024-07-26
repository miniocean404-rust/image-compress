use std::panic::set_hook;

use backtrace::Backtrace;

pub fn register_panic_hook() {
    // set_hook: 注册一个自定义 panic 钩子，替换以前注册的钩子，这个钩子将在线程调用 panic 之前被调用
    // 钩子会收到一个 PanicInfo 结构，其中包含有关 panic 起源的信息，包括传递给 panic! 的 payload 和 panic 的来源代码位置。
    set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::new();
        println!("panic 信息: {:#?}\n", panic_info,);
        println!("调用栈:\n{:?}\n", backtrace);
    }));
}
