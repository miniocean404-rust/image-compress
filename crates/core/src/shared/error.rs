use std::io;

// 模拟从其他库中导入的错误类型

#[derive(thiserror::Error, Debug)]
pub enum TauriError {
    #[error("没有获取窗口")]
    NoWindow,

    #[error("没有获取路径")]
    NoPath,

    #[error("没有获取 Option 值")]
    NoValue,

    #[error("JSON 错误: {0}")]
    Json(serde_json::Error),
}

impl serde::Serialize for TauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OptionError {
    #[error("没有获取 Option 值")]
    NoValue,
}

#[derive(thiserror::Error, Debug)]
pub enum WebpError {
    #[error("webp 解码错误")]
    DecodeError,

    #[error("webp 编码错误")]
    EncodeError,

    #[error("webp 压缩出错")]
    CompressError,

    #[error("webp io 写入错误")]
    WriteError(#[from] io::Error),
}
