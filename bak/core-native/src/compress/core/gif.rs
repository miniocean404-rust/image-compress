use std::ffi::CString;
use std::fmt;
use std::os::raw::{c_int, c_void};

#[derive(Debug, Clone)]
pub struct CaesiumError {
    pub message: String,
    pub code: u32,
}

impl fmt::Display for CaesiumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.message, self.code)
    }
}

pub fn lossy_gif(input_path: &str, output_path: &str) -> Result<(), CaesiumError> {
    unsafe {
        let input_file = libc::fopen(
            CString::new(input_path)
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20406,
                })?
                .as_ptr(),
            CString::new("r")
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20407,
                })?
                .as_ptr(),
        );
        let output_file = libc::fopen(
            CString::new(output_path)
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20408,
                })?
                .as_ptr(),
            CString::new("w+")
                .map_err(|e| CaesiumError {
                    message: e.to_string(),
                    code: 20409,
                })?
                .as_ptr(),
        );

        let input_stream = gifsicle::Gif_ReadFile(input_file);
        libc::fclose(input_file);

        let padding: [*mut c_void; 7] = [std::ptr::null_mut(); 7];
        let loss = (100 - 30) as c_int;

        let gc_info = gifsicle::Gif_CompressInfo { flags: 0, loss, padding };
        let write_result = gifsicle::Gif_FullWriteFile(input_stream, &gc_info, output_file);
        libc::fclose(output_file);

        match write_result {
            1 => Ok(()),
            _ => Err(CaesiumError {
                message: "GIF optimization failed!".to_string(),
                code: 20410,
            }),
        }
    }
}
