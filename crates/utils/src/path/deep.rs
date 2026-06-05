use std::fs;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

pub fn get_deep_dirs(pattern: &str, dir: &str, max_deep: usize) -> anyhow::Result<Vec<String>> {
    let mut builder = GlobSetBuilder::new();
    let glob = Glob::new(pattern)?;
    builder.add(glob);

    let glob_set = builder.build()?;

    let mut result = vec![];

    let walk_dir = WalkDir::new(dir)
        .max_depth(max_deep)
        // 是否跟踪符号链接
        .follow_links(true);

    for entry in walk_dir
        .into_iter()
        // 过滤隐藏文件
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry?;

        if glob_set.is_match(entry.path()) {
            result.push(entry.path().display().to_string());
        }
    }

    Ok(result)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

// 递归文件夹
pub async fn read_dir_path_buf(path: &str) -> anyhow::Result<Vec<PathBuf>> {
    fn deep_dir(p: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let entries = fs::read_dir(p)?;
        let mut vec_path_buf = Vec::new();

        for entry in entries {
            let entry = entry?;
            let meta = entry.metadata()?;

            if meta.is_file() {
                vec_path_buf.push(entry.path());
            } else if meta.is_dir() {
                vec_path_buf.extend(deep_dir(&entry.path())?);
            }
        }

        Ok(vec_path_buf)
    }

    deep_dir(Path::new(path))
}
