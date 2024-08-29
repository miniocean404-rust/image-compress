use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{error, info};

use crate::utils::mime::{get_filetype_from_path, SupportedFileTypes};

use super::core::{
    jpeg::{self},
    png, webp,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImageCompression {
    pub name: String,

    pub state: CompressState,

    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub mem: Vec<u8>,

    #[serde(default, rename(serialize = "type", deserialize = "type"))]
    pub file_type: SupportedFileTypes,

    #[serde(default)]
    pub quality: i8,

    // serialize 把 before_size 序列化为 origin | deserialize 把 origin 反序列化为 before_size
    #[serde(default, rename(serialize = "origin", deserialize = "origin"))]
    pub before_size: u64,

    #[serde(default, rename(serialize = "compress", deserialize = "compress"))]
    pub after_size: u64,

    // 浮点数转化为 json 传递必须是 f64 否则精度丢失
    #[serde(default)]
    pub rate: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // 序列化枚举为小写字符串
pub enum CompressState {
    #[default]
    Compressing,
    Done,
}

impl ImageCompression {
    pub fn new(path: String, quality: i8) -> Self {
        let file_type = get_filetype_from_path(&path);

        let path_buf = PathBuf::from(&path);
        let file_name = path_buf.file_name();
        let metadata = fs::metadata(&path_buf);

        match (file_name, metadata) {
            (Some(name), Ok(before_size)) => {
                Self {
                    name: name.to_string_lossy().into_owned(),
                    file_type,
                    quality,
                    before_size: before_size.len(),
                    path,
                    // 没有初始化的字段使用默认值
                    ..Default::default()
                }
            }
            _ => {
                panic!("文件名有问题 或 文件尺寸获取失败")
            }
        }
    }

    pub async fn start_mem_compress(&mut self, is_cover: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CompressState::Done;

        let mem = match self.file_type {
            SupportedFileTypes::Jpeg => {
                let file = fs::read(&self.path)?;
                // let mem = jpeg::lib_mozjpeg_sys::lossless::optimize_lossy_jpeg(&file, self.quality, false, ChromaSubsampling::CS420).unwrap();
                let mem = jpeg::lib_mozjpeg_sys::optimize_lossless_jpeg(&file, false);
                mem.to_vec()
            }
            SupportedFileTypes::Png => {
                // 有损加无损压缩 Png
                let file = fs::read(&self.path)?;
                let lossless_mem = png::lossless::to_mem(&file)?;
                png::lossy::to_mem(&lossless_mem, self.quality)?
            }
            SupportedFileTypes::WebP => webp::to_mem(&self.path, false, self.quality as f32)?,
            SupportedFileTypes::Gif => Vec::new(),
            SupportedFileTypes::Unknown => {
                error!("不支持的类型");
                return Ok(());
            }
        };

        self.mem = mem;
        self.after_size = self.mem.len() as u64;

        let before_size = self.before_size as f64;
        let after_size = self.after_size as f64;

        self.rate = (((before_size - after_size) / before_size) * 10000.0).round() / 100.0;

        if is_cover {
            let mut output_file = File::create(&self.path).await?;
            info!(output_file=?output_file);
            output_file.write_all(self.mem.as_slice()).await?;
        };

        Ok(())
    }
}
