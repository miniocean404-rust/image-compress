use std::path::PathBuf;

use crate::{
    compress::{index::ImageCompression, utils::dir::glob_dir},
    shared::error::OptionError,
};

pub fn get_compress_infos(dir: &str) -> anyhow::Result<Vec<ImageCompression>> {
    let path = PathBuf::from(dir);
    let files = glob_dir("*.{png,webp,gif,jpg,jpeg}", path.to_str().ok_or(OptionError::NoValue)?).map_err(|_| OptionError::NoValue)?;

    let infos = files
        .into_iter()
        .map(|file| ImageCompression::new(file, 80).unwrap())
        .collect::<Vec<ImageCompression>>();

    Ok(infos)
}
