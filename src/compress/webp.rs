use anyhow::{Ok, Result};
use libwebp_sys::{WebPDecodeRGBA, WebPEncodeRGBA, WebPGetInfo};

pub fn encode_webp(input_image: &[u8], width: i32, height: i32, quality: f32) -> Result<Vec<u8>> {
    unsafe {
        let mut out_buf = std::ptr::null_mut();

        let stride = width * 4;

        let len = WebPEncodeRGBA(
            input_image.as_ptr(),
            width,
            height,
            stride,
            quality,
            &mut out_buf,
        );

        Ok(std::slice::from_raw_parts(out_buf, len as usize).into())
    }
}

pub fn decode_webp(buf: &[u8]) -> Result<Vec<u8>> {
    let mut width = 0;
    let mut height = 0;
    let len = buf.len();

    unsafe {
        WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height);
        let out_buf = WebPDecodeRGBA(buf.as_ptr(), len, &mut width, &mut height);

        Ok(std::slice::from_raw_parts(out_buf, (width * height * 4).try_into().unwrap()).into())
    }
}
