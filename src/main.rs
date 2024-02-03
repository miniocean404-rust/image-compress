use std::path::PathBuf;

use image_compress::{
    compress::{index::ImageCompression, utils::dir::glob_dir},
    shared::error::OptionError,
    utils::log::tracing::init_tracing,
};
use tracing::info;

fn main() -> anyhow::Result<()> {
    let _guard = init_tracing("./logs");

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
    let infos = get_compress_infos("D:\\soft-dev\\code\\rust\\image-compress\\image")?;

    // info!("{:?}", infos);
    dbg!(infos);

    anyhow::Ok(())
}

fn get_compress_infos(dir: &str) -> anyhow::Result<Vec<ImageCompression>> {
    let path = PathBuf::from(dir);
    let files = glob_dir("*.{png,webp,gif,jpg,jpeg}", path.to_str().ok_or(OptionError::NoValue)?).map_err(|_| OptionError::NoValue)?;

    let infos = files
        .into_iter()
        .map(|file| ImageCompression::new(file, 80).unwrap())
        .collect::<Vec<ImageCompression>>();

    Ok(infos)
}

fn async_thread_stop() {
    // warn!("异步线程停止了");
}
