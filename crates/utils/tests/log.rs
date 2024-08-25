#![allow(unused_imports)]
use tracing::{event, info, instrument, level_filters::LevelFilter, trace, Instrument, Level};
use tracing_futures::WithSubscriber;
use tracing_serde::AsSerde;
use utils::log::tracing::LogUtil;

#[test]
fn log_with_default() -> anyhow::Result<()> {
    LogUtil::init();
    tokio_main()?;
    Ok(())
}

#[test]
fn log_with_file() -> anyhow::Result<()> {
    let _guard = LogUtil::init_with_layer("../../logs", LevelFilter::INFO)?;
    tokio_main()?;
    Ok(())
}

fn tokio_main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        // 开启所有特性
        .enable_all()
        // 构建 runtime
        .build()?;

    // 等价于 #[tokio::main()]
    rt.block_on(async_main())?;

    Ok(())
}

// 参考：https://docs.rs/tracing/latest/tracing/attr.instrument.html
// 通过 instrument 属性，直接让整个函数或方法进入 span 区间，且适用于异步函数 async fn fn_name(){}
#[tracing::instrument(name = "my_span", level = "info")]
#[instrument]
async fn async_main() -> anyhow::Result<()> {
    info!("{:?}", "测试日志");

    // #[instrument]属性表示函数整体在一个span区间内，因此函数内的每一个event信息中都会额外带有函数参数
    // 在函数中，只需发出日志即可
    event!(Level::TRACE, custom_params = 1, "key1: value1");
    trace!(custom_params = 2, "key2: value3");
    info!(custom_params = 3, "key3: value2");
    Ok(())
}
