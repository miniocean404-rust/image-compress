#[derive(Clone, Debug, Default)]
pub enum SupportedFileTypes {
    Jpeg,
    Png,
    WebP,
    Avif,

    // 为 Default 宏设置默认值
    #[default]
    Unknown,
}

impl From<&str> for SupportedFileTypes {
    fn from(mime: &str) -> Self {
        match mime {
            "image/jpeg" => SupportedFileTypes::Jpeg,
            "image/png" => SupportedFileTypes::Png,
            "image/webp" => SupportedFileTypes::WebP,
            "image/avif" => SupportedFileTypes::Avif,
            // "image/gif" => SupportedFileTypes::Gif,
            _ => SupportedFileTypes::Unknown,
        }
    }
}
