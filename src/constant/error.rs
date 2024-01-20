use thiserror::Error;

// 模拟从其他库中导入的错误类型

#[derive(Error, Debug)]
pub enum Error {
    #[error("tokio 发送消息失败")]
    SendMessageError,

    #[error("类型转换错误")]
    Type2Error,
}
