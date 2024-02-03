use infer::Type;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // 序列化枚举为小写字符串
pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,

    // 为 Default 宏设置默认值
    #[default]
    Unknown,
}

pub fn get_filetype_from_path(file_path: &str) -> SupportedFileTypes {
    match infer::get_from_path(file_path) {
        Ok(v) => match v {
            Some(ft) => match_supported_filetypes(ft),
            None => SupportedFileTypes::Unknown,
        },
        Err(_) => SupportedFileTypes::Unknown,
    }
}

pub fn get_filetype_from_memory(buf: &[u8]) -> SupportedFileTypes {
    match infer::get(buf) {
        Some(ft) => match_supported_filetypes(ft),
        None => SupportedFileTypes::Unknown,
    }
}

fn match_supported_filetypes(ft: Type) -> SupportedFileTypes {
    match ft.mime_type() {
        "image/jpeg" => SupportedFileTypes::Jpeg,
        "image/png" => SupportedFileTypes::Png,
        "image/gif" => SupportedFileTypes::Gif,
        "image/webp" => SupportedFileTypes::WebP,
        _ => SupportedFileTypes::Unknown,
    }
}
