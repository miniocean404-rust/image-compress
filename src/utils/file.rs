use std::{fs, path::PathBuf};

use anyhow::Result;

pub async fn read_dir_path_buf(path: &str) -> Result<Vec<PathBuf>> {
    fn deep_dir(p: &str) -> Result<Vec<PathBuf>> {
        let entries = fs::read_dir(p)?;

        let vec_path_buf = entries
            .filter_map(|entry| entry.ok())
            .flat_map(|entry| {
                let meta = entry.metadata().ok()?;
                if meta.is_file() {
                    Some(vec![entry.path()])
                } else if meta.is_dir() {
                    deep_dir(entry.path().to_str().unwrap()).ok()
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        Ok(vec_path_buf)
    }

    deep_dir(path)
}
