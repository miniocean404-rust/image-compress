use std::{any::Any, fmt::Debug};

#[cfg(feature = "avif")]
pub mod avif;

#[cfg(feature = "jpeg")]
pub mod jpeg;

#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "webp")]
pub mod webp;

// 无压缩编码，只有解码
#[cfg(feature = "tiff")]
pub mod tiff;

// # 暂未实现
#[cfg(feature = "gif")]
pub mod gif;

pub trait OptionsTrait: Any + Debug + Clone {
    fn as_any(&self) -> &dyn Any;
}
