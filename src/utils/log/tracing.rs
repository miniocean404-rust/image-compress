// https://rust-book.junmajinlong.com/ch102/tracing.html

use std::io;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{
        self,
        format::{Format, Full},
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Registry,
};

use super::time::LocalTimer;

// 直接初始化，采用默认的Subscriber，默认只输出INFO、WARN、ERROR级别的日志
// tracing_subscriber::fmt::init();

pub fn init_tracing() -> WorkerGuard {
    // 使用 tracing_appender，指定日志的输出目标位置
    // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/

    let file_appender = tracing_appender::rolling::daily("./logs", "tracing.log");
    // 如果 non_blocking 不在 main 中，需要把 guard 返回给 main
    let (_non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let tty = fmt::layer().with_writer(io::stdout).event_format(get_formart(true));
    let file = fmt::layer().with_writer(_non_blocking).event_format(get_formart(false));

    let registry = Registry::default().with(tty).with(file);
    registry.init();

    // 初始化并设置日志格式(定制和筛选日志)
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::TRACE)
    //     .init(); // 初始化并将SubScriber设置为全局SubScriber

    guard
}

fn get_formart(color: bool) -> Format<Full, LocalTimer> {
    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    fmt::format()
        .with_level(true)
        .with_timer(LocalTimer)
        // 是否展示目标调用的包路径
        .with_target(false)
        // 是否显示源代码路径
        .with_file(true)
        // 是否显示源代码行号
        .with_line_number(true)
        // 否显示源代码路径、行号总开关
        .with_source_location(true)
        .with_ansi(color)
        // 是否展示线程名、线程 id
        .with_thread_names(true)
        .with_thread_ids(true)
}

// 通过 instrument 属性，直接让整个函数或方法进入span区间，且适用于异步函数async fn fn_name(){}
// 参考：https://docs.rs/tracing/latest/tracing/attr.instrument.html
// #[tracing::instrument(level = "info")]
// #[instrument]
// fn test_trace(n: i32) {
//     // #[instrument]属性表示函数整体在一个span区间内，因此函数内的每一个event信息中都会额外带有函数参数
//     // 在函数中，只需发出日志即可
//     event!(Level::TRACE, answer = 42, "trace2: test_trace");
//     trace!(answer = 42, "trace1: test_trace");
//     info!(answer = 42, "info1: test_trace");
// }
