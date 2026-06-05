use chrono::{FixedOffset, Utc};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

// 用来格式化日志的输出时间格式
#[derive(Debug, Clone, Copy)]
pub struct LocalTimer;

// 常量函数
const fn east8() -> Option<FixedOffset> {
    FixedOffset::east_opt(8 * 60 * 60)
}

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        // 使用chrono::Local::now()的效率相对会差一些，因为每次获取时间都要探测本机的时区。因此可改进为使用Offset的方式，明确指定时区，无需探测
        // write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"));

        let now = Utc::now().with_timezone(&east8().unwrap());
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}
