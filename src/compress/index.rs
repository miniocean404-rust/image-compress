use std::{
    fmt::{self},
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::{error, info};

use crate::shared::error::OptionError;

use super::{
    core::{
        jpeg::{self},
        png, webp,
    },
    utils::mime::{get_filetype_from_path, SupportedFileTypes},
};

#[derive(Clone, Default, Serialize, Deserialize)]
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
    pub quality: i32,

    // serialize 把 before_size 序列化为 origin | deserialize 把 origin 反序列化为 before_size
    #[serde(default, rename(serialize = "origin", deserialize = "origin"))]
    pub before_size: u64,

    #[serde(default, rename(serialize = "compress", deserialize = "compress"))]
    pub after_size: usize,

    #[serde(default)]
    pub rate: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] // 序列化枚举为小写字符串
pub enum CompressState {
    Ready,
    #[default]
    Compressing,
    Done,
}

impl ImageCompression {
    pub fn new(path: String, quality: i32) -> Result<Self, Box<dyn std::error::Error>> {
        let file_type = get_filetype_from_path(&path);

        let path_buf = PathBuf::from(&path);
        let file_name = path_buf.file_name().ok_or(OptionError::NoValue)?.to_string_lossy().to_string();

        let before_size = fs::metadata(&path_buf)?.len();

        Ok(Self {
            name: file_name,
            file_type,
            quality,
            before_size,
            path,
            // 没有初始化的字段使用默认值
            ..Default::default()
        })
    }

    pub async fn start_mem_compress(&mut self, is_cover: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CompressState::Done;

        let mem = match self.file_type {
            SupportedFileTypes::Jpeg => {
                let file = fs::read(&self.path).unwrap();
                // let mem = jpeg::lib_mozjpeg_sys::lossless::optimize_lossy_jpeg(&file, self.quality, false, ChromaSubsampling::CS420).unwrap();
                let mem = jpeg::lib_mozjpeg_sys::optimize_lossless_jpeg(&file, false).unwrap();
                mem.to_vec()
            }
            SupportedFileTypes::Png => {
                // 有损加无损压缩 Png
                let file = fs::read(&self.path).unwrap();
                let lossless_mem = png::lossless::to_mem(&file).unwrap();
                png::lossy::to_mem(&lossless_mem, self.quality).unwrap()
            }
            SupportedFileTypes::WebP => webp::to_mem(&self.path, false, self.quality as f32).unwrap(),
            SupportedFileTypes::Gif => Vec::new(),
            SupportedFileTypes::Unknown => {
                error!("不支持的类型");
                return Ok(());
            }
        };

        self.mem = mem;
        self.after_size = self.mem.len();

        self.rate = format!(
            "{:.2}",
            ((((self.before_size as f32 - self.after_size as f32) / self.before_size as f32) * 1000.0).round() / 1000.0) * 100.0
        );

        if is_cover {
            let mut output_file = File::create(&self.path).await?;
            info!(output_file=?output_file);
            output_file.write_all(self.mem.as_slice()).await?;
        };

        Ok(())
    }
}

impl fmt::Debug for ImageCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_map().entries(self.enable_info_map.iter());
        // f.debug_list().entries(self.enable_user_vec.iter()).finish()
        // writeln!(f, "自定义格式化 {} ", self.mem.len()).unwrap_or(());

        f.debug_struct("ImageCompression")
            .field("name", &self.name)
            .field("state", &self.state)
            .field("path", &self.path)
            .field("mem", &self.mem.len())
            .field("file_type", &self.file_type)
            .field("quality", &self.quality)
            .field("before_size", &self.before_size)
            .field("after_size", &self.after_size)
            .field("rate", &self.rate)
            .finish()
    }
}

impl fmt::Display for ImageCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_map().entries(self.enable_info_map.iter());
        // f.debug_list().entries(self.enable_user_vec.iter()).finish()

        f.debug_struct("ImageCompression")
            .field("name", &self.name)
            .field("state", &self.state)
            .field("path", &self.path)
            .field("mem", &self.mem.len())
            .field("file_type", &self.file_type)
            .field("quality", &self.quality)
            .field("before_size", &self.before_size)
            .field("after_size", &self.after_size)
            .field("rate", &self.rate)
            .finish()
    }
}
