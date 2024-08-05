use napi_derive::napi;

#[napi]
// 获取 目标三元组（target triple）。目标三元组是一个字符串，用于标识目标系统的体系结构、供应商和操作系统。它是由三个部分组成的：
// arch-vendor-os
pub fn get_target_triple() -> napi::Result<String> {
    let info = include_str!(concat!(env!("OUT_DIR"), "/triple.txt"));
    Ok(info.to_string())
}
