use mozjpeg_sys::*;
use std::{mem, ptr, slice};

struct JPEGOptimizer {
    srcinfo: jpeg_decompress_struct,
    dstinfo: jpeg_compress_struct,
}

impl JPEGOptimizer {
    unsafe fn new() -> JPEGOptimizer {
        JPEGOptimizer {
            srcinfo: mem::zeroed(),
            dstinfo: mem::zeroed(),
        }
    }
}

impl Drop for JPEGOptimizer {
    fn drop(&mut self) {
        unsafe {
            jpeg_destroy_decompress(&mut self.srcinfo);
            jpeg_destroy_compress(&mut self.dstinfo);
        }
    }
}

// This function losslessly optimizes jpegs.
// Based on the jpegtran.c example program in libjpeg.
pub fn optimize_lossless_jpeg(bytes: &[u8], keep_metadata: bool) -> std::thread::Result<&mut [u8]> {
    unsafe {
        std::panic::catch_unwind(|| {
            let mut info = JPEGOptimizer::new();
            let mut err = create_error_handler();
            info.srcinfo.common.err = &mut err;
            jpeg_create_decompress(&mut info.srcinfo);
            jpeg_mem_src(&mut info.srcinfo, bytes.as_ptr(), bytes.len() as c_ulong);

            // 原本没有
            if keep_metadata {
                jpeg_save_markers(&mut info.srcinfo, 0xFE, 0xFFFF);
                for m in 0..16 {
                    jpeg_save_markers(&mut info.srcinfo, 0xE0 + m, 0xFFFF);
                }
            }

            info.dstinfo.optimize_coding = 1;
            info.dstinfo.common.err = &mut err;
            jpeg_create_compress(&mut info.dstinfo);
            jpeg_read_header(&mut info.srcinfo, 1);

            let src_coef_arrays = jpeg_read_coefficients(&mut info.srcinfo);
            jpeg_copy_critical_parameters(&info.srcinfo, &mut info.dstinfo);

            let mut buf = ptr::null_mut();
            let mut outsize: c_ulong = 0;

            jpeg_mem_dest(&mut info.dstinfo, &mut buf, &mut outsize);
            // jpeg_set_quality(&mut info.dstinfo, 80, false as boolean);
            jpeg_write_coefficients(&mut info.dstinfo, src_coef_arrays);

            if keep_metadata {
                write_metadata(&mut info.srcinfo, &mut info.dstinfo);
            }

            jpeg_finish_compress(&mut info.dstinfo);
            jpeg_finish_decompress(&mut info.srcinfo);

            slice::from_raw_parts_mut(buf, outsize as usize)
        })
    }
}

pub fn optimize_lossy_jpeg(bytes: &[u8], quality: i32, keep_metadata: bool) -> std::thread::Result<&mut [u8]> {
    unsafe {
        std::panic::catch_unwind(|| {
            let mut info = JPEGOptimizer::new();
            let mut err = create_error_handler();
            info.srcinfo.common.err = &mut err;
            jpeg_create_decompress(&mut info.srcinfo);
            jpeg_mem_src(&mut info.srcinfo, bytes.as_ptr(), bytes.len() as c_ulong);

            // 原本没有
            if keep_metadata {
                jpeg_save_markers(&mut info.srcinfo, 0xFE, 0xFFFF);
                for m in 0..16 {
                    jpeg_save_markers(&mut info.srcinfo, 0xE0 + m, 0xFFFF);
                }
            }

            info.dstinfo.optimize_coding = 1;
            info.dstinfo.common.err = &mut err;
            jpeg_create_compress(&mut info.dstinfo);

            // 获取源图片信息
            jpeg_read_header(&mut info.srcinfo, 1);
            let width = info.srcinfo.image_width;
            let height = info.srcinfo.image_height;
            let color_space = info.srcinfo.jpeg_color_space;
            info.srcinfo.out_color_space = color_space;

            let input_components = match color_space {
                mozjpeg_sys::J_COLOR_SPACE::JCS_GRAYSCALE => 1,
                mozjpeg_sys::J_COLOR_SPACE::JCS_RGB => 3,
                mozjpeg_sys::J_COLOR_SPACE::JCS_YCbCr => 3,
                mozjpeg_sys::J_COLOR_SPACE::JCS_CMYK => 4,
                mozjpeg_sys::J_COLOR_SPACE::JCS_YCCK => 4,
                mozjpeg_sys::J_COLOR_SPACE::JCS_EXT_RGBA => 4,
                _ => 3,
            };

            info.dstinfo.image_width = width;
            info.dstinfo.image_height = height;
            info.dstinfo.in_color_space = color_space;
            info.dstinfo.input_components = input_components;

            // Chroma subsampling 是一种在数字图像和视频压缩中使用的技术。它涉及到减少颜色信息的采样率，以便在保持相对较高的图像质量的同时降低数据量。
            // 这种技术通常用于视频编解码和存储中，以降低文件大小并提高传输效率。Chroma subsampling 通常使用 4:4:4、4:2:2 和 4:2:0 等比例来描述不同的采样方式。

            // 开始解码
            jpeg_start_decompress(&mut info.srcinfo);
            // output_components 代表颜色类型
            let row_stride = width as usize * info.srcinfo.output_components as usize;
            let buffer_size = row_stride * height as usize;
            let mut buffer = vec![0u8; buffer_size];

            // 开始扫描图片信息给解压缩上
            while info.srcinfo.output_scanline < info.srcinfo.output_height {
                let offset = info.srcinfo.output_scanline as usize * row_stride;
                let mut jsamparray = [buffer[offset..].as_mut_ptr()];
                //Crash on very first call of this function on Android
                jpeg_read_scanlines(&mut info.srcinfo, jsamparray.as_mut_ptr(), 1);
            }

            // let src_coef_arrays = jpeg_read_coefficients(&mut info.srcinfo);
            // jpeg_copy_critical_parameters(&info.srcinfo, &mut info.dstinfo);

            let mut buf = ptr::null_mut();
            let mut outsize: c_ulong = 0;

            // 用于设置 JPEG 压缩后的数据输出目标为内存缓冲区。
            jpeg_mem_dest(&mut info.dstinfo, &mut buf, &mut outsize);
            // jpeg_write_coefficients(&mut info.dstinfo, src_coef_arrays);

            // 设置压缩信息为默认值
            jpeg_set_defaults(&mut info.dstinfo);

            let row_stride = info.dstinfo.image_width as usize * info.dstinfo.input_components as usize;
            info.dstinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;

            // 第三个参数表示是否启用平滑功能
            jpeg_set_quality(&mut info.dstinfo, quality, false as boolean);

            jpeg_start_compress(&mut info.dstinfo, true as boolean);

            if keep_metadata {
                write_metadata(&mut info.srcinfo, &mut info.dstinfo);
            }

            while info.dstinfo.next_scanline < info.dstinfo.image_height {
                let offset = info.dstinfo.next_scanline as usize * row_stride;
                let jsamparray = [buffer[offset..].as_ptr()];
                jpeg_write_scanlines(&mut info.dstinfo, jsamparray.as_ptr(), 1);
            }

            jpeg_finish_compress(&mut info.dstinfo);
            jpeg_finish_decompress(&mut info.srcinfo);

            slice::from_raw_parts_mut(buf, outsize as usize)
        })
    }
}

unsafe fn create_error_handler() -> jpeg_error_mgr {
    let mut err: jpeg_error_mgr = mem::zeroed();
    jpeg_std_error(&mut err);
    err.error_exit = Some(unwind_error_exit);
    err.emit_message = Some(silence_message);
    err
}

// 处理错误退出消息
extern "C-unwind" fn unwind_error_exit(cinfo: &mut jpeg_common_struct) {
    let message = unsafe {
        let err = cinfo.err.as_ref().unwrap();

        match err.format_message {
            Some(fmt) => {
                let buffer = mem::zeroed();
                fmt(cinfo, &buffer);
                let len = buffer.iter().take_while(|&&c| c != 0).count();
                String::from_utf8_lossy(&buffer[..len]).into()
            }
            None => format!("libjpeg error: {}", err.msg_code),
        }
    };
    std::panic::resume_unwind(Box::new(message))
}

extern "C-unwind" fn silence_message(_cinfo: &mut jpeg_common_struct, _level: c_int) {}

pub(crate) unsafe fn write_metadata(src_info: &mut jpeg_decompress_struct, dst_info: &mut jpeg_compress_struct) {
    let mut marker = src_info.marker_list;

    while !marker.is_null() {
        jpeg_write_marker(dst_info, (*marker).marker as i32, (*marker).data, (*marker).data_length);
        marker = (*marker).next;
    }
}
