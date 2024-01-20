use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use anyhow::{Ok, Result};
use tokio::task::JoinSet;
use image_compress::{
    compress::png::lossy_png,
    utils::{file::read_dir_path_buf, log::tracing::init_tracing},
};
use tracing::info;

fn main() -> Result<()> {
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
    let path = "D:\\soft-dev\\code\\work\\davinci\\davinci-web\\assets\\image";
    // let path = "/Users/user/Desktop/work-code/front-end/davinci-web/assets/image";
    // let path = "image";

    let res = read_dir_path_buf(path).await?.into_iter().enumerate();
    let mut set = JoinSet::new();

    for (index,path_buf) in res {
        if let Some(ext) = path_buf.extension() {
            if ext == "png" {
                set.spawn(async move {
                    let path = path_buf.as_path().to_str().unwrap();
                    let out = Path::new("dist").join(path_buf.file_name().unwrap());
                    let out = out.to_str().unwrap();

                    info!("path :{:?}", path);
                    info!("out :{:?}", out);
                    info!("数量 :{:?}", index);
                    lossy_png(path, out).await.unwrap();
                });
            }
        }
    }

    while let Some(_) = set.join_next().await {}

    Ok(())
}

fn async_thread_stop() {
    // warn!("异步线程停止了");
}
