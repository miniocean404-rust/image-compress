use image_compress::support::SupportedFileTypes;
use napi_derive::napi;

// string_enum 转化为字符串枚举
#[allow(non_camel_case_types)]
#[napi(string_enum)]
pub enum Kind {
    jpeg,
    png,
    gif,
    webp,
    avif,
    unknown,
}

impl From<SupportedFileTypes> for Kind {
    fn from(s: SupportedFileTypes) -> Self {
        match s {
            SupportedFileTypes::Jpeg => Kind::jpeg,
            SupportedFileTypes::Png => Kind::png,
            SupportedFileTypes::WebP => Kind::webp,
            SupportedFileTypes::Avif => Kind::avif,
            SupportedFileTypes::Unknown => Kind::unknown,
        }
    }
}
