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
        // 遍历所有条目并忽略可能出现的任何错误
        .filter_map(Result::ok)
    {
        if glob_set.is_match(entry.path()) {
            result.push(entry.path().display().to_string());
        }
    }

    Ok(result)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
