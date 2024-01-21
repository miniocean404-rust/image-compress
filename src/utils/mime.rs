use infer::Type;

pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Gif,
    WebP,
    Unknown,
}

pub fn get_filetype_from_path(file_path: &str) -> SupportedFileTypes {
    match infer::get_from_path(file_path) {
        Ok(v) => match v {
            None => SupportedFileTypes::Unknown,
            Some(ft) => match_supported_filetypes(ft),
        },
        Err(_) => SupportedFileTypes::Unknown,
    }
}

pub fn get_filetype_from_memory(buf: &[u8]) -> SupportedFileTypes {
    match infer::get(buf) {
        None => SupportedFileTypes::Unknown,
        Some(ft) => match_supported_filetypes(ft),
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
