use tracing::LogUtil;

pub mod time;
pub mod tracing;

pub fn config() -> LogUtil {
    LogUtil::default()
}
