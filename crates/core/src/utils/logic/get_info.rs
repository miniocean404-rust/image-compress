use std::path::PathBuf;

use utils::path::deep::get_deep_dirs;

use crate::compress::index::ImageCompression;

pub fn get_compress_infos(dir: &str) -> anyhow::Result<Vec<ImageCompression>> {
    let path_buf = PathBuf::from(dir);
    let path = path_buf.to_str().unwrap_or("");

    let files = get_deep_dirs("*.{png,webp,gif,jpg,jpeg}", path, 5)?;

    let infos = files
        .into_iter()
        .map(|file| ImageCompression::new(file, 80).unwrap())
        .collect::<Vec<ImageCompression>>();

    Ok(infos)
}
