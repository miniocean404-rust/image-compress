#[cfg(feature = "png")]
pub mod png;

#[cfg(feature = "jpeg")]
pub mod jpeg;

#[cfg(feature = "webp")]
pub mod webp;

#[cfg(feature = "avif")]
pub mod avif;

#[cfg(feature = "gif")]
pub mod gif;
