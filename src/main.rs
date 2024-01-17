use anyhow::Result;
use image_compress::utils::log::tracing::init_tracing;

fn main() -> Result<()> {
    let guard = init_tracing();

    let rt = tokio::runtime::Builder::new_multi_thread()
        // 开启所有特性
        .enable_all()
        // 监听线程停止
        .on_thread_stop(async_thread_stop)
        // 构建 runtime
        .build()?;

    // 等价于 #[tokio::main()]
    rt.block_on(async_main())?;

    Ok(())
}

async fn async_main() -> Result<()> {
    Ok(())
}

fn async_thread_stop() {
    // warn!("异步线程停止了");
}
