// https://rust-book.junmajinlong.com/ch102/tracing.html

use std::{env, fmt::Debug, io, vec};

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

#[derive(Debug)]
pub struct LogUtil<T = LocalTimer> {
    pub(crate) path: String,
    pub(crate) level: Level,
    pub(crate) color: bool,
    pub(crate) show_level: bool,
    pub(crate) timer: T,
    pub(crate) target: bool,
    pub(crate) file: bool,
    pub(crate) line_number: bool,
    pub(crate) source_location: bool,
    pub(crate) thread_names: bool,
    pub(crate) thread_ids: bool,
    pub(crate) filter_creates: Vec<String>,
}

impl Default for LogUtil<LocalTimer> {
    fn default() -> Self {
        let cur_create = env!("CARGO_PKG_NAME").replace('-', "_");

        Self {
            path: "./logs".to_string(),
            level: Level::TRACE,
            color: true,
            show_level: true,
            // 能赋值的原因是它是一个空结构体 struct LocalTimer
            timer: LocalTimer,
            target: false,
            file: true,
            line_number: true,
            source_location: true,
            thread_names: true,
            thread_ids: true,
            filter_creates: vec![cur_create],
        }
    }
}

impl<T> LogUtil<T> {
    // 设置输出日志级别
    pub fn with_level(self, level: Level) -> Self {
        Self { level, ..self }
    }

    // 设置日志输出路径
    pub fn with_path(self, path: &str) -> Self {
        Self {
            path: path.to_string(),
            ..self
        }
    }

    // 是否展示日志 level 信息
    pub fn with_show_level(self, show_level: bool) -> Self {
        Self { show_level, ..self }
    }

    // 是否展示终端 ansi 编码日志颜色
    pub fn with_color(self, color: bool) -> Self {
        Self { color, ..self }
    }

    // 设置日志时间格式
    pub fn with_timer(self, timer: T) -> LogUtil<T> {
        Self { timer, ..self }
    }

    // 是否展示目标调用的包路径
    pub fn with_target(self, target: bool) -> Self {
        Self { target, ..self }
    }

    // 是否显示源代码路径
    pub fn with_file(self, file: bool) -> Self {
        Self { file, ..self }
    }

    // 是否显示源代码行号
    pub fn with_line_number(self, line_number: bool) -> Self {
        Self {
            line_number,
            ..self
        }
    }

    // 否显示源代码路径、行号总开关
    pub fn with_source_location(self, source_location: bool) -> Self {
        Self {
            source_location,
            ..self
        }
    }

    // 是否展示线程名
    pub fn with_thread_names(self, thread_names: bool) -> Self {
        Self {
            thread_names,
            ..self
        }
    }

    // 是否展示线程 id
    pub fn with_thread_ids(self, thread_ids: bool) -> Self {
        Self { thread_ids, ..self }
    }

    pub fn with_filter_create(mut self, filter_creates: Vec<&str>) -> Self {
        let mut filter_creates: Vec<String> = filter_creates
            .iter()
            .map(|create| create.to_string())
            .collect();
        self.filter_creates.append(&mut filter_creates);
        self
    }
}

impl LogUtil {
    // 直接初始化初始化并设置日志格式(定制和筛选日志)
    pub fn init(&self) {
        tracing_subscriber::fmt()
            // .with_env_filter(filter)
            .event_format(self.get_formart(self.color))
            .json()
            .with_writer(io::stdout)
            .init(); // 初始化并将SubScriber设置为全局SubScriber
    }

    pub fn init_with_layer(&self) -> anyhow::Result<WorkerGuard> {
        // 使用 tracing_appender，指定日志的输出目标位置
        // 参考: https://docs.rs/tracing-appender/0.2.0/tracing_appender/

        // 设置日志过滤器，只输出项目下的不含第三方库的日志 过滤器格式：https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
        // tauri 中不能添加，会导致程序无法启动
        // filter 另一种写法：let filter: EnvFilter = "utils,log".into();
        let mut filter = EnvFilter::from_default_env();
        for create in &self.filter_creates {
            let directive = format!("{}={}", create, self.level);
            filter = filter.add_directive(directive.parse()?);
        }
        filter = filter.add_directive(LevelFilter::from_level(self.level).into());

        let file_appender = tracing_appender::rolling::daily(&self.path, "tracing.log");
        // guard 必须返回给主函数
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let tty = tracing_subscriber::fmt::layer()
            .with_writer(io::stdout)
            .event_format(self.get_formart(self.color));
        // .json()

        let file = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .event_format(self.get_formart(false));

        let registry = Registry::default().with(filter).with(tty).with(file);

        registry.init();

        Ok(guard)
    }

    fn get_formart(&self, color: bool) -> Format<Full, LocalTimer> {
        // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
        // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
        fmt::format()
            .with_level(self.show_level)
            .with_timer(self.timer)
            .with_target(self.target)
            .with_source_location(self.source_location)
            .with_file(self.file)
            .with_line_number(self.line_number)
            .with_ansi(color)
            // .json()
            .with_thread_names(self.thread_names)
            .with_thread_ids(self.thread_ids)
    }
}
