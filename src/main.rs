use std::io;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::{error, info};

use image_compress::compress::gif::lossy_gif;
use image_compress::compress::png::lossy_png;
use image_compress::compress::webp::webp_compress;
use image_compress::constant::error::Error;
use image_compress::utils::file::read_dir_path_buf;
use image_compress::utils::log::tracing::init_tracing;

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

async fn async_main() -> Result<()> {
    lossy_gif(
        "D:\\soft-dev\\code\\rust\\image-compress\\image\\gif\\a.gif",
        "output.gif",
    )
    .unwrap();
    Ok(())
}

async fn start_compress() -> Result<()> {
    let path = "D:\\soft-dev\\code\\work\\davinci\\davinci-web\\assets\\image";
    // let path = "/Users/user/Desktop/work-code/front-end/davinci-web/assets/image";
    // let path = "image";

    let res = read_dir_path_buf(path).await?.into_iter().enumerate();
    let mut set = JoinSet::new();

    for (index, path_buf) in res {
        let ext = path_buf
            .extension()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "获取扩展名错误"))?
            .to_str()
            .ok_or(Error::Type2Error)?;

        let input = path_buf.as_path().to_string_lossy().to_string();
        let output = Path::new("dist")
            .join(path_buf.file_name().ok_or(Error::Type2Error)?)
            .to_string_lossy()
            .to_string();

        let input = Arc::new(RwLock::new(input));
        let arc_input = Arc::clone(&input);
        let output = Arc::new(RwLock::new(output));
        let arc_output = Arc::clone(&output);
        match ext {
            "png" => {
                set.spawn(async move {
                    let input = arc_input.read().await;
                    let output = arc_output.read().await;

                    info!("path :{:?}", &input);
                    info!("out :{:?}", output);
                    info!("数量 :{:?}", index);

                    lossy_png(&input, &output).await
                });
            }
            "webp" => {
                set.spawn(async move {
                    let input = arc_input.read().await;
                    let output = arc_output.read().await;

                    info!("path :{:?}", &input);
                    info!("out :{:?}", output);
                    info!("数量 :{:?}", index);

                    webp_compress(&input, &output)
                });
            }
            _ => {
                continue;
            }
        };
    }

    while let Some(thread) = set.join_next().await {
        match thread? {
            Ok(_) => {}
            Err(err) => {
                error!("线程错误 {}", err)
            }
        }
    }

    Ok(())
}

fn async_thread_stop() {
    // warn!("异步线程停止了");
}
