use napi::Env;
use napi_derive::napi;
use once_cell::sync::OnceCell;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

static CHROME_LAYER_ONCE: OnceCell<bool> = OnceCell::new();

#[napi]
pub fn init_custom_trace_subscriber(
    mut env: Env,
    trace_out_file_path: Option<String>,
) -> napi::Result<()> {
    CHROME_LAYER_ONCE.get_or_init(|| {
        let mut layer = ChromeLayerBuilder::new().include_args(true);
        if let Some(trace_out_file) = trace_out_file_path {
            layer = layer.file(trace_out_file);
        }

        let (chrome_layer, guard) = layer.build();

        let layer = chrome_layer.with_filter(filter::filter_fn(|metadata| {
            !metadata.target().contains("cranelift") && !metadata.name().contains("log ")
        }));

        tracing_subscriber::registry()
            .with(layer)
            .try_init()
            .expect("失败的注册 tracing subscriber");

        // node 程序结束自动清理 guard
        env.add_env_cleanup_hook(guard, |flush_guard| {
            flush_guard.flush();
            drop(flush_guard);
        })
        .expect("Node-API 无法清理初始化数据");

        true
    });

    Ok(())
}

pub fn init_default_trace_subscriber() {
    let _unused = tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_ansi(true)
        // .with_env_filter(EnvFilter::from_env("SWC_LOG"))
        .pretty()
        .try_init();
}
