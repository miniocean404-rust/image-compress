// https://rust-book.junmajinlong.com/ch102/tracing.html

use std::{env, fmt::Debug, io};

use tracing::{level_filters::LevelFilter, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{
        self,
        format::{Format, Full},
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

use super::time::LocalTimer;

#[derive(Debug, Default)]
pub struct LogUtil {
    pub path: String,
}

impl LogUtil {
    // 直接初始化初始化并设置日志格式(定制和筛选日志)
    pub fn init() {
        tracing_subscriber::fmt()
            // .with_env_filter(filter)
            .json()
            .with_writer(io::stdout)
            .with_max_level(Level::TRACE)
            .init(); // 初始化并将SubScriber设置为全局SubScriber
    }

    pub fn init_with_layer(path: &str, level: LevelFilter) -> anyhow::Result<WorkerGuard> {
        // 使用 tracing_appender，指定日志的输出目标位置
        // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/

        // 设置日志过滤器，只输出项目下的不含第三方库的日志 过滤器格式：https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
        // tauri 中不能添加，会导致程序无法启动
        // filter 另一种写法：let filter: EnvFilter = "utils,log".into();
        let my_create = env!("CARGO_PKG_NAME").replace('-', "_");
        let filter = EnvFilter::from_default_env()
            .add_directive(level.into())
            .add_directive(my_create.parse()?)
            .add_directive("log".parse()?);

        let file_appender = tracing_appender::rolling::daily(path, "tracing.log");
        // 如果 non_blocking 不在 main 中，需要把 guard 返回给 main
        let (_non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let tty = tracing_subscriber::fmt::layer()
            .with_writer(io::stdout)
            .event_format(Self::get_formart(true));
        // .json()

        let file = tracing_subscriber::fmt::layer()
            .with_writer(_non_blocking)
            .event_format(Self::get_formart(false));

        let registry = Registry::default().with(filter).with(tty).with(file);

        registry.init();

        Ok(guard)
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
            // .json()
            // 是否展示线程名、线程 id
            .with_thread_names(true)
            .with_thread_ids(true)
    }
}
