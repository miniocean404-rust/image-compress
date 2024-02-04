use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

// build.rs
extern crate napi_build;

// #[cfg(all(not(feature = "swc_v1"), not(feature = "swc_v2")))]
// compile_error!("Please enable swc_v1 or swc_v2 feature");

fn main() {
    let out_dir = env::var("OUT_DIR").expect("应该存在环境变量 outdir");
    let dest_path = Path::new(&out_dir).join("triple.txt");

    let mut f = BufWriter::new(File::create(dest_path).expect("失败的创建 triple text"));
    write!(f, "{}", env::var("TARGET").expect("应该指定目标")).expect("写入 [目标三元组] 失败");

    // napi_build::setup();
}
