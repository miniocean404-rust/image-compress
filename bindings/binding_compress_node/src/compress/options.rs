use napi::bindgen_prelude::Object;
use napi_derive::napi;

#[napi(object)]
pub struct MozJpegOptions {
    pub quality: u8,
}

impl From<Object> for MozJpegOptions {
    fn from(value: Object) -> Self {
        MozJpegOptions {
            quality: value.get("quality").unwrap().unwrap_or(0),
        }
    }
}

#[napi(object)]
pub struct OxiPngOptions {}

#[napi(object)]
pub struct ImageQuantOptions {}

#[napi(object)]
pub struct WebPOptions {}

#[napi(object)]
pub struct AvifOptions {}
