use std::thread;
use std::time::Duration;

use tracing::warn;

pub enum SecsTime {
    MAX,
    Time(u64),
}

pub fn set_sleep(secs: SecsTime) {
    match secs {
        SecsTime::MAX => {
            thread::sleep(Duration::from_secs(u64::MAX));
        }
        SecsTime::Time(secs) => {
            thread::sleep(Duration::from_secs(secs));
        }
    }
}

pub async fn exit_guard() -> anyhow::Result<()> {
    loop {
        tokio::select! {
           // 如果取消biased，挑选的任务顺序将随机，可能会导致分支中的断言失败
           // 默认情况下，select!会伪随机公平地轮询每一个分支，如果确实需要让select!按照任务书写顺序去轮询，
           biased;
           _ = tokio::signal::ctrl_c() => {
              warn!("ctrl-c 接收, 执行结束");
              // std::process::exit(0);
              return Ok(())
          }
          // 你的代码中的问题是在 else => { break; } 分支中使用了 break; 语句，
          // 这导致了编译时错误 "this loop never actually loops"。
          // 这是因为 tokio::select! 宏期望被嵌套在一个循环内，而在你的代码中，break; 导致循环无限结束，这与 tokio::select! 的预期不符。
          else => {}
        }
    }
}
