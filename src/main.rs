use std::error::Error;

use image_compress::{
    compress::{jpg::lossless_jpeg, png::lossy_png},
    utils::{file::read_dir_path_buf, log::tracing::init_tracing},
};
use tracing::info;

fn main() -> Result<(), Box<dyn Error>> {
    let _guard = init_tracing();

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

async fn async_main() -> anyhow::Result<()> {
    // let path = "D:\\soft-dev\\code\\work\\davinci\\davinci-web\\assets\\image";
    let path = "image";

    let res = read_dir_path_buf(path).await?;
    info!(res = ?res, "读取文件夹");

    lossy_png("image/png/time-icon.png", "dist/png/test.png")?;

    Ok(())
}

fn async_thread_stop() {
    // warn!("异步线程停止了");
}
