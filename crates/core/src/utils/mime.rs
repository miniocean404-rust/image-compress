use serde::{Deserialize, Serialize};

use utils::file::mime::{get_mime_for_memory, get_mime_for_path};

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

impl From<&str> for SupportedFileTypes {
    fn from(mime: &str) -> Self {
        match mime {
            "image/jpeg" => SupportedFileTypes::Jpeg,
            "image/png" => SupportedFileTypes::Png,
            "image/gif" => SupportedFileTypes::Gif,
            "image/webp" => SupportedFileTypes::WebP,
            _ => SupportedFileTypes::Unknown,
        }
    }
}

pub fn get_filetype_from_path(file_path: &str) -> SupportedFileTypes {
    get_mime_for_path(file_path).unwrap_or("").into()
}

pub fn get_filetype_from_memory(buf: &[u8]) -> SupportedFileTypes {
    get_mime_for_memory(buf).into()
}
